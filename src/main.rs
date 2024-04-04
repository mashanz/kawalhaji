fn main() -> std::io::Result<()> {
    actix_web::rt::System::new().block_on(async { kawalhaji::run()?.await })?;
    Ok(())
}
