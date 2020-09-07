//! This structure supports different systems running at different times during the game.

use amethyst::{
    core::{
        SystemBundle,
        ArcThreadPool,
        deferred_dispatcher_operation::{
            AddBundle,
            DispatcherOperation,
            AddSystem,
        },
    },
    ecs::{
        DispatcherBuilder,
        Dispatcher,
        System,
        World,
        WorldExt,
    },
    error::{
        Error,
    },
    DataInit,
    DataDispose,
};

/// A named dispatcher that will dispatch a group of systems.
pub struct DispatchGroup<'a, 'b> {
    /// The name of the dispatch group
    pub name: &'a str,
    /// The dispatcher for the group
    pub dispatcher: Dispatcher<'a, 'b>,
}

/// A version of Amethyst's Game Data that supports groups of systems.
///
/// The lifetimes are for the systems inside and can be `'static` unless a system has a borrowed
/// field.
pub struct GameData<'a, 'b> {
    /// The base dispatcher always runs
    pub base_dispatcher: Option<Dispatcher<'a, 'b>>,
    /// The running of these dispatchers can be controlled
    /// by the user in the call to update.
    pub dispatchers: Vec<DispatchGroup<'a, 'b>>
}

impl<'a, 'b> GameData<'a, 'b> {
    /// Creates new game data.
    ///
    /// # Parameters
    ///
    /// - `dispatcher`: the base Amethyst ECS Dispatcher to use within the game.
    /// - `dispatch_groups`: a vector of optional dispatch groups.
    ///
    /// # Returns
    ///
    /// This function returns [DispatchGroupBuilder](struct.GameData.html).
    pub fn new(dispatcher: Dispatcher<'a, 'b>, dispatch_groups: Vec<DispatchGroup<'a, 'b>>) -> Self {
        GameData {
            base_dispatcher: Some(dispatcher),
            dispatchers: dispatch_groups,
        }
    }
    /// Runs the set of systems specified by the slice of dispatcher_id,
    /// as well as the base dispatcher's systems.
    ///
    /// # Parameters
    ///
    /// - `world`: the ECS `World` for this application.
    /// - `dispatcher_id_list`: a vector of optional dispatch groups by index.
    ///
    /// It seems a little bit weird to wrap the dispatcher id's in options,
    /// but it means you can write code like:
    /// ```rust
    ///   update(world, [game_data.get_dispatcher_id("a"), game_data.get_dispatcher_id("b")]
    /// ```
    pub fn update(&mut self, world: &World, dispatcher_id_list: &[Option<usize>]) {
        if let Some(dispatcher) = &mut self.base_dispatcher {
            //+ log::info!("Running base dispatcher.");
            dispatcher.dispatch(&world)
        }
        for &optional_dispatcher_id in dispatcher_id_list {
            if let Some(dispatcher_id) = optional_dispatcher_id {
                //+ log::info!("running optional dispatcher '{:?}'", self.dispatchers[dispatcher_id].name);
                self.dispatchers[dispatcher_id].dispatcher.dispatch(&world)
            }
        }
    }
    /// Gets the dispatcher_id for a given name.
    ///
    /// # Parameters
    ///
    /// - `name`: the name of the dispatch group.
    ///
    /// # Returns
    ///
    /// This method returns the ID (index)
    /// of the given dispatcher, if present,
    /// and `None` if there is no dispatcher with that name.
    pub fn get_dispatcher_id(&self, name: &str) -> Option<usize> {
        for i in 0..self.dispatchers.len() {
            if self.dispatchers[i].name.eq(name) {
                return Some(i)
            }
        }
        return None
    }
    /// Disposes game data, dropping the dispatchers.
    ///
    /// # Parameters
    ///
    /// - `world`: the ECS `World` for this application.
    #[allow(unused)]
    pub fn dispose(&mut self, mut world: &mut World) {
        if let Some(dispatcher) = self.base_dispatcher.take() {
            dispatcher.dispose(&mut world);
        }
        while !self.dispatchers.is_empty() {
            let dispatch_group = self.dispatchers.remove(0);
            dispatch_group.dispatcher.dispose(&mut world);
        }
    }
}
impl<'a, 'b> DataDispose for GameData<'a, 'b> {
    fn dispose(&mut self, world: &mut World) {
        self.dispose(world);
    }
}

/// The builder for dispatch groups.
pub struct DispatchGroupBuilder<'a, 'b> {
    pub name: &'a str,
    pub dispatcher_builder: DispatcherBuilder<'a, 'b>,
    pub dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
}

impl<'a, 'b> DispatchGroupBuilder<'a, 'b> {
    /// Creates a new dispatch group builder.
    ///
    /// # Parameters
    ///
    /// - `name`: A unique string by which to identify the dispatch group
    ///    in calls to [GameData::update](struct.GameData.html#method.update).
    ///
    /// # Returns
    ///
    /// This function returns a new [DispatchGroupBuilder](struct.DispatchGroupBuilder.html)
    /// with no operations.
    #[allow(dead_code)]
    pub fn new(name: &'static str) -> Self {
        DispatchGroupBuilder {
            name,
            dispatcher_builder: DispatcherBuilder::default(),
            dispatcher_operations: vec![],
        }
    }
    /// Adds a single system to the dispatch group.
    ///
    /// __Note:__ all dependencies must be added before you add the system.
    ///
    /// # Parameters
    ///
    /// - `system`: The system that is to be added to the game loop.
    /// - `name`: A unique string to identify the system by. This is used for
    ///         dependency tracking. This name may be empty `""` string in which
    ///         case it cannot be referenced as a dependency.
    /// - `dependencies`: A list of named system that _must_ have completed running
    ///                 before this system is permitted to run.
    ///                 This may be an empty list if there are no dependencies.
    ///
    /// # Returns
    ///
    /// This function returns [DispatchGroupBuilder](struct.DispatchGroupBuilder.html) after it has modified it.
    ///
    /// # Type Parameters
    ///
    /// - `S`: A type that implements the `System` trait.
    /// - `N`: A type that can be converted into String and can be cloned.
    ///
    /// # Panics
    ///
    /// If two system are added that share an identical name, this function will panic.
    /// Empty names are permitted, and this function will not panic if more then two are added.
    ///
    /// If a dependency is referenced (by name), but has not previously been added this
    /// function will panic.
    #[allow(unused)]
    pub fn with_system<S,N>(mut self, system: S, name: N, dependencies: &[N]) -> Self
        where
            S: for<'c> System<'c> + 'static + Send,
            N: Into<String> + Clone,
    {
        let dispatcher_operation = make_dispatcher_operation(system, name, dependencies);
        self.dispatcher_operations.push(dispatcher_operation);
        self
    }
    /// Adds a bundle of systems to the dispatch group.
    ///
    /// A bundle is a container for registering a bunch of ECS systems at once.
    ///
    /// # Parameters
    ///
    /// - `world`: The `World` that contains all resources.
    /// - `bundle`: The bundle to add.
    ///
    /// # Returns
    ///
    /// This function returns [DispatchGroupBuilder](struct.DispatchGroupBuilder.html) after it has modified it,
    /// wrapped in a `Result`.
    ///
    /// # Errors
    ///
    /// This function creates systems, which use any number of dependent crates or APIs, which
    /// could result in any number of errors.
    /// See each individual bundle for a description of the errors it could produce.
    ///
    #[allow(dead_code)]
    pub fn with_bundle<B>(mut self, bundle: B) -> Result<Self, Error>
        where
            B: SystemBundle<'a, 'b> + 'static,
    {
        self.dispatcher_operations
            .push(Box::new(AddBundle { bundle }));
        Ok(self)
    }
}

/// Structure to build a GameData system from the given
/// base and dispatch builders.
///
pub struct GameDataBuilder<'a, 'b> {
    /// The builder for the dispatcher that always runs.
    pub base_builder: DispatcherBuilder<'a, 'b>,
    /// The deferred operations for the base dispatcher
    base_dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    /// The builders for the optional dispatchers.
    pub dispatch_builders: Vec<DispatchGroupBuilder<'a, 'b>>,
    // (the builders each have their own list of deferred operations)
}


impl <'a, 'b> Default for GameDataBuilder<'a, 'b> {
    fn default() -> Self { GameDataBuilder::new() }
}

impl<'a, 'b> GameDataBuilder<'a, 'b> {
    /// Creates a new GameDataBuilder.
    pub fn new() -> Self {
        GameDataBuilder {
            base_builder: DispatcherBuilder::new(),
            base_dispatcher_operations: vec![],
            dispatch_builders: vec![]
        }
    }
    /// Appends a system to base dispatcher.
    ///
    /// The appended system will always run.
    #[allow(unused)]
    pub fn with_system<S,N>(mut self, system: S, name: N, dependencies: &[N]) -> Self
        where
            S: for<'c> System<'c> + 'static + Send,
            N: Into<String> + Clone,
    {
        let dispatcher_operation = make_dispatcher_operation(system, name, dependencies);
        self.base_dispatcher_operations.push(dispatcher_operation);
        self
    }
    /// Appends a bundle to the base dispatcher.
    ///
    /// The systems in the bundle will always run.
    pub fn with_bundle<B>(mut self, bundle: B) -> Result<Self, Error>
        where
            B: SystemBundle<'a, 'b> + 'static,
    {
        self.base_dispatcher_operations
            .push(Box::new(AddBundle { bundle }));
        Ok(self)
    }
    /// Appends a dispatch group builder to the game data builder.
    ///
    /// These systems can be turned on or off by parameters
    /// to the GameData's `update` call.
    #[allow(unused)]
    pub fn add_dispatch_builder(mut self, name: &'a str, dispatch_builder: DispatcherBuilder<'a, 'b>) -> Self {
        self.dispatch_builders.push(
            DispatchGroupBuilder {
                name,
                dispatcher_builder: dispatch_builder,
                dispatcher_operations: vec![],
            }
        );
        self
    }
    /// Adds a dispatch group to the system description.
    #[allow(dead_code)]
    pub fn with_dispatch_group(mut self, dispatch_group_builder: DispatchGroupBuilder<'a, 'b>) -> Result<Self, Error> {
        self.dispatch_builders.push(dispatch_group_builder);
        Ok(self)
    }
}

impl<'a, 'b>DataInit<GameData<'a, 'b>> for GameDataBuilder<'a, 'b> {
    /// Builds a GameData dispatcher set from the given info.
    fn build(self, world: &mut World) -> GameData<'a, 'b> {
        #[cfg(not(no_threading))]
            let pool = (*world.read_resource::<ArcThreadPool>()).clone();

        let base_dispatcher = {
            let mut dispatcher_builder = self.base_builder;

            self.base_dispatcher_operations
                .into_iter()
                .try_for_each(|dispatcher_operation| {
                    dispatcher_operation.exec(world, &mut dispatcher_builder)
                })
                .unwrap_or_else(|e| panic!("Failed to set up dispatcher: {}", e));

            #[cfg(not(no_threading))]
                let mut base_dispatcher = dispatcher_builder.with_pool(pool.clone()).build();
            #[cfg(no_threading)]
                let mut base_dispatcher = dispatcher_builder.build();
            base_dispatcher.setup(world);
            base_dispatcher
        };

        let mut dispatch_group: Vec<DispatchGroup> = vec![];
        for dispatch_group_builder in self.dispatch_builders {
            let mut dispatch_builder = dispatch_group_builder.dispatcher_builder;
            dispatch_group_builder.dispatcher_operations
                .into_iter()
                .try_for_each(|dispatcher_operation| {
                    dispatcher_operation.exec(world, &mut dispatch_builder)
                })
                .unwrap_or_else(|e| panic!("Failed to set up dispatcher: {}", e))
            ;
            #[cfg(not(no_threading))]
                let dispatcher = dispatch_builder.with_pool(pool.clone()).build();
            #[cfg(no_threading)]
                let dispatcher = dispatch_builder.build();
            dispatch_group.push(DispatchGroup {
                name: dispatch_group_builder.name,
                dispatcher
            })
        }

        GameData::new(
            base_dispatcher,
            dispatch_group,
        )
    }
}

/// Builds a system add request.
///
/// Code copied from `amethyst_core/src/deferred_dispatcher_operation.rs`.
#[allow(unused)]
fn make_dispatcher_operation<'a, 'b, S, N>(system: S, name: N, dependencies: &[N]) -> Box<dyn DispatcherOperation<'a, 'b> + 'static>
    where
        S: for<'c> System<'c> + 'static + Send,
        N: Into<String> + Clone,
{
    let name = Into::<String>::into(name);
    let dependencies = dependencies
        .iter()
        .map(Clone::clone)
        .map(Into::<String>::into)
        .collect::<Vec<String>>();
    Box::new(AddSystem {
        system,
        name,
        dependencies,
    }) as Box<dyn DispatcherOperation<'a, 'b> + 'static>
}
