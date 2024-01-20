use router_macro::component;

/// The app's router
pub struct Router;

#[component(Router = "/")]
fn Home() {
    let route = Router::Home();
}

#[component(Router = "/login?name={name}&password={password}")]
fn Login(name: String, password: String) {
    let route = Router::Logout();
}

#[component(Router = "/logout")]
fn Logout() {
    let route = Router::Login("name".to_string(), "password".to_string());
}

#[component(Router = "/blog/{id}")]
fn Blog() {}

#[test]
fn print_routes() {
    // Only works on non-wasm targets, which is generally okay?
    for info in support::ROUTES.iter().rev() {
        let info = info();
        println!("{}", info.route);
    }
}

mod support {
    use linkme::distributed_slice;
    use std::any::TypeId;
    pub struct RouteInfo {
        // The router info
        pub route: &'static str,

        // The TypeId of the router itself
        pub id: TypeId,
    }
    #[distributed_slice]
    pub static ROUTES: [fn() -> RouteInfo];
}
