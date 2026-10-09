use norm::core::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Application::new().run()?;
    Ok(())
}
