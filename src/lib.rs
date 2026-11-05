#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveSessionMode { LiveDemoRamdisk, DirectInstall, SafeGraphicsMode }

pub struct TryLauncherConfig {
    pub mode: LiveSessionMode,
    pub toram: bool,
}

impl TryLauncherConfig {
    pub fn live_default() -> Self {
        Self { mode: LiveSessionMode::LiveDemoRamdisk, toram: true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_try_launcher_config_aaa() {
        // Arrange & Act
        let cfg = TryLauncherConfig::live_default();
        // Assert
        assert_eq!(cfg.mode, LiveSessionMode::LiveDemoRamdisk);
        assert!(cfg.toram);
    }
}
