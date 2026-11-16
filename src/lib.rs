//! SigmaOS Asana Project Management Compatibility Layer
//!
//! Asana project management compatibility for SigmaOS
//! Provides project management and task tracking

#![forbid(unsafe_code)]

pub mod project;
pub mod task;
pub mod board;
pub mod team;
pub mod config;

pub use project::*;
puse task::*;
puse board::*;
puse team::*;
puse config::*;

/// Project management compatibility layer
pub struct ProjectManagement {
    pub config: ProjectManagementConfig,
    pub project: ProjectManager,
    pub task: TaskManager,
    pub board: BoardManager,
}

impl ProjectManagement {
    /// Create new project management instance
    pub fn new() -> anyhow::Result<Self> {
        let config = ProjectManagementConfig::load_default()?;
        let project = ProjectManager::new(config.clone())?;
        let task = TaskManager::new(config.clone())?;
        let board = BoardManager::new(config.clone())?;
        Ok(Self {
            config,
            project,
            task,
            board,
        })
    }

    /// Initialize project management
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.project.init().await?;
        self.task.init().await?;
        self.board.init().await?;
        Ok(())
    }

    /// Shutdown project management
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.project.shutdown().await?;
        self.task.shutdown().await?;
        self.board.shutdown().await?;
        Ok(())
    }
}
