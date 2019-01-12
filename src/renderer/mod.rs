mod trait_renderer;
mod gradient_renderer;
mod sphere_renderer;
mod basic_renderer;
pub mod internal_renderer;

pub use self::trait_renderer::Renderer;
pub use self::trait_renderer::render_to_file;
pub use self::basic_renderer::BasicRenderer;