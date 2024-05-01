use std::io::{self, stdout, Stdout};

use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use crate::cpu::{Cpu, CpuState};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub struct App {
    cpu: Cpu,
    cycles: u32,
    exit: bool,
    previous_state: Option<CpuState>,
    current_state: Option<CpuState>,
}

impl App {
    pub fn new(cpu: Cpu) -> App {
        let curr = CpuState::new(&cpu);

        App { cpu, cycles: 0, exit: false, previous_state: None, current_state: Some(curr)}
    }

    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(' ') => self.execute_instruction(),
            _ => {}
        }
    }

    fn render_frame(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
                Constraint::Length(1),
            ])
            .direction(Direction::Vertical)
            .split(frame.size());

        let cpu_layouts = Layout::default()
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .direction(Direction::Horizontal)
            .split(layout[0]);

        let previous_cpu_block = Block::default()
            .title(Title::from(" Previous State ".bold()).alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Rgb(0, 255, 0)).bg(Color::Rgb(0, 0, 0)));

        let current_cpu_block = Block::default()
            .title(Title::from(" Current State ".bold()).alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Rgb(0, 255, 0)).bg(Color::Rgb(0, 0, 0)));
        
        let prev_cpu_string: String = match &self.previous_state {
            Some(cpu_state) => 
                format!("{}", cpu_state)
            ,
            None => String::from(""),
        };

        let previous_cpu_state = Paragraph::new(prev_cpu_string)
            .style(Style::default().fg(Color::Rgb(0, 255, 0)).bg(Color::Rgb(0, 0, 0)))
            .alignment(Alignment::Center)
            .block(previous_cpu_block);
        
        let curr_cpu_string: String = match &self.current_state {
            Some(cpu_state) => 
                format!("{}", cpu_state)
            ,
            None => String::from(""),
        };

        let current_cpu_state = Paragraph::new(curr_cpu_string) 
            .style(Style::default().fg(Color::Rgb(0, 255, 0)).bg(Color::Rgb(0, 0, 0)))
            .alignment(Alignment::Center)
            .block(current_cpu_block);

        let title = Paragraph::new(" RS-6502 Emulator ")
            .alignment(Alignment::Center);

        frame.render_widget(previous_cpu_state, cpu_layouts[0]);
        frame.render_widget(current_cpu_state, cpu_layouts[1]);
        frame.render_widget(title, layout[0]);

        let instruction = Paragraph::new("Press 'q' to quit, 'space' to execute an instruction")
            .style(Style::default().fg(Color::Rgb(0, 255, 0)).bg(Color::Black))
            .alignment(Alignment::Center);

        frame.render_widget(instruction, layout[2])
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn execute_instruction(&mut self) {
        self.previous_state = self.current_state.take();
        self.cycles += self.cpu.execute() as u32;
        self.current_state = Some(CpuState::new(&self.cpu));
    }
}

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}