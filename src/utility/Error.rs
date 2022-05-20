
#[macro_export]
macro_rules! error_print {
    ($stdout:expr, $error:expr) => {
        $stdout
            .queue(SetForegroundColor(Color::DarkRed))?
            .queue(Print(">Error| "))?
            .queue(ResetColor)?
            .queue(Print(format!("{}\n", $error)))?
            .queue(ResetColor)?;
        $stdout.flush();
    }
}