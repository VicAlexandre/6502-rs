use std::io::{self, stdout, Stdout};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use crate::cpu::Cpu;
use crate::cpu_state::CpuState;
use crate::instruction::Instruction;

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub struct App {
    cpu: Cpu,
    exit: bool,
    previous_state: Option<CpuState>,
    current_state: Option<CpuState>,
    memory: Option<Vec<u8>>,
    memory_index: usize,
    num_memory_lines: u8,
    stack: Option<Vec<u8>>,
    prev_instruction: Option<Instruction>,
    curr_instruction: Option<Instruction>,
}

impl App {
    pub fn new(cpu: Cpu) -> App {
        let curr = CpuState::new(&cpu);
        let stack = cpu.stack.get_stack();
        let curr_instruction = Instruction::new(&cpu);

        App {
            cpu,
            exit: false,
            previous_state: None,
            current_state: Some(curr),
            memory: None,
            memory_index: 0,
            num_memory_lines: 40,
            stack: Some(stack),
            prev_instruction: None,
            curr_instruction: Some(curr_instruction),
        }
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
            KeyCode::Char(' ') => self.execute_instruction_and_update_state(),
            KeyCode::Down | KeyCode::Char('s') => self.scroll_down_memory(),
            KeyCode::Up | KeyCode::Char('w') => self.scroll_up_memory(),
            _ => {}
        }
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        // The layout of the application
        let layout = Layout::default()
            .constraints([
                Constraint::Length(11),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .direction(Direction::Vertical)
            .split(frame.size());

        self.num_memory_lines = layout[1].height as u8 - 3;
        self.memory = Some(self.cpu.memory.get_ram(self.memory_index, self.num_memory_lines as usize * 16));

        // Split the CPU and Memory layout
        let cpu_layouts = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .direction(Direction::Horizontal)
            .split(layout[0]);

        let mem_layouts = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .direction(Direction::Horizontal)
            .split(layout[1]);

        // Split the stack and instructions layout
        let stack_instructions_layouts = Layout::default()
            .constraints([Constraint::Length(19), Constraint::Fill(1)])
            .direction(Direction::Vertical)
            .split(mem_layouts[1]);

        let instructions_layouts = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .direction(Direction::Horizontal)
            .split(stack_instructions_layouts[1]);

        // Split previous and current CPU state
        let previous_cpu_block = Block::default()
            .title(Title::from(" Previous State ".bold()).alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            );

        let current_cpu_block = Block::default()
            .title(Title::from(" Current State ".bold()).alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            );

        // Create the blocks for the memory layout
        let ram_page_block: Block = Block::default()
            .title(Title::from("  RAM Page 0x0000 - 0x00FF  ".bold()).alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            );

        // Create the blocks for the stack and instructions layout
        let stack_page_block: Block = Block::default()
            .title(Title::from("  Stack Memory  ".bold()).alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            );

        let prev_instruction_block: Block = Block::default()
            .title(Title::from("  Previous  ".bold()).alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            );

        let curr_instruction_block: Block = Block::default()
            .title(Title::from("  Current  ".bold()).alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            );

        // Getting the paragraph for the CPU states
        let prev_cpu_string: String = match &self.previous_state {
            Some(cpu_state) => format!("{}", cpu_state),
            None => String::from(""),
        };

        let previous_cpu_state = Paragraph::new(prev_cpu_string)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            )
            .alignment(Alignment::Center)
            .block(previous_cpu_block);

        let curr_cpu_string: String = match &self.current_state {
            Some(cpu_state) => format!("{}", cpu_state),
            None => String::from(""),
        };

        let current_cpu_state = Paragraph::new(curr_cpu_string)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            )
            .alignment(Alignment::Center)
            .block(current_cpu_block);

        let prev_instruction_string = match &self.prev_instruction {
            Some(instruction) => format!("{}", instruction),
            None => String::from(""),
        };

        let curr_instruction_string = match &self.curr_instruction {
            Some(instruction) => format!("{}", instruction),
            None => String::from(""),
        };

        let prev_instruction_state = Paragraph::new(prev_instruction_string)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            )
            .alignment(Alignment::Left)
            .block(prev_instruction_block);

        let curr_instruction_state = Paragraph::new(curr_instruction_string)
            .style(
                Style::default()
                    .fg(Color::Rgb(0, 255, 0))
                    .bg(Color::Rgb(0, 0, 0)),
            )
            .alignment(Alignment::Left)
            .block(curr_instruction_block);

        // Getting the paragraph for the interactive instructions
        let instruction = Paragraph::new("<SPACE> - Execute Instruction | <q> - Quit | <↑/w> - Scroll RAM Up | <↓/s> - Scroll RAM Down")
            .style(Style::default().fg(Color::Rgb(0, 255, 0)).bg(Color::Black))
            .alignment(Alignment::Center);

        let title = Paragraph::new(" MOS 6502 Emulator ").alignment(Alignment::Center);

        let instructions_title = Paragraph::new(" Instructions ").alignment(Alignment::Center);

        let memory_table = Table::default()
            .block(ram_page_block)
            .header(Row::new([
                " ", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
            ]))
            .widths(
                (0..17)
                    .map(|_| Constraint::Percentage(100 / 17))
                    .collect::<Vec<Constraint>>(),
            )
            .rows((0..self.num_memory_lines as usize).map(|i| {
                if (16 * i as u32 + self.memory_index as u32) >= 0x10000 {
                    return Row::new(vec![Cell::from("")]);
                }

                let mut cells = vec![Cell::from(format!(
                    "${:03X}_",
                    (16 * i + self.memory_index) >> 4
                ))];
                for j in 0..16 {
                    let index = i * 16 + j;
                    let value = format!("${:02X}", self.memory.as_ref().unwrap()[index]);

                    let cell = if index + self.memory_index == self.cpu.pc as usize {
                        Cell::from(value).style(
                            Style::default()
                                .bg(Color::Rgb(0, 255, 0))
                                .fg(Color::Rgb(0, 0, 0)),
                        )
                    } else {
                        Cell::from(value)
                    };

                    cells.push(cell);
                }

                Row::new(cells)
            }));

        let stack_table = Table::default()
            .block(stack_page_block)
            .header(Row::new([
                " ", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
            ]))
            .widths(
                (0..17)
                    .map(|_| Constraint::Percentage(100 / 17))
                    .collect::<Vec<Constraint>>(),
            )
            .rows((0..self.stack.as_ref().unwrap().len() / 16).map(|i| {
                let mut cells = vec![Cell::from(format!("${:1X}_", (i * 16) >> 4))];
                for j in 0..16 {
                    let index = i * 16 + j;
                    let value = format!("${:02X}", self.stack.as_ref().unwrap()[index]);

                    let cell = if index == self.cpu.stack.sp as usize {
                        Cell::from(value).style(
                            Style::default()
                                .bg(Color::Rgb(0, 255, 0))
                                .fg(Color::Rgb(0, 0, 0)),
                        )
                    } else {
                        Cell::from(value)
                    };

                    cells.push(Cell::from(cell));
                }

                Row::new(cells)
            }));

        frame.render_widget(previous_cpu_state, cpu_layouts[0]);
        frame.render_widget(current_cpu_state, cpu_layouts[1]);

        frame.render_widget(memory_table, mem_layouts[0]);
        frame.render_widget(stack_table, stack_instructions_layouts[0]);
        frame.render_widget(prev_instruction_state, instructions_layouts[0]);
        frame.render_widget(curr_instruction_state, instructions_layouts[1]);

        frame.render_widget(title, layout[0]);
        frame.render_widget(instructions_title, stack_instructions_layouts[1]);
        frame.render_widget(instruction, layout[2]);
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn execute_instruction_and_update_state(&mut self) {
        self.previous_state = self.current_state.take();
        self.prev_instruction = self.curr_instruction.take();

        let cycles_used = self.cpu.execute() as u32;
        let mut new_state = CpuState::new(&self.cpu);
        new_state.cycles = self.previous_state.as_ref().unwrap().cycles + cycles_used;

        let new_instruction = Instruction::new(&self.cpu);

        self.curr_instruction = Some(new_instruction);
        self.current_state = Some(new_state);

        self.memory = Some(self.cpu.memory.get_ram(self.memory_index, self.num_memory_lines as usize * 16));
        self.stack = Some(self.cpu.stack.get_stack());
    }

    fn scroll_down_memory(&mut self) {
        if self.memory_index + self.num_memory_lines as usize * 16 >= 0x10000 {
            self.memory_index = 0;
        } else {
            self.memory_index = self.memory_index + self.num_memory_lines as usize * 16
        }

        self.memory = Some(self.cpu.memory.get_ram(self.memory_index, self.num_memory_lines as usize * 16));
    }

    fn scroll_up_memory(&mut self) {
        if self.memory_index as i32 - self.num_memory_lines as i32 * 16 < 0 {
            if self.memory_index == 0 {
                self.memory_index = 0x10000 - self.num_memory_lines as usize * 16;
            } else {
                self.memory_index = 0;
            }
        } else {
            self.memory_index = self.memory_index - self.num_memory_lines as usize * 16;
        }

        self.memory = Some(self.cpu.memory.get_ram(self.memory_index, self.num_memory_lines as usize * 16));
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
