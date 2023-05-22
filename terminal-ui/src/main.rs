use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Gauge},
    layout::{Layout, Constraint, Direction},
    Terminal,
    style::{Color, Modifier, Style},
};
use crossterm::{
    event::{self, Event, KeyCode, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use sysinfo::{CpuExt, ProcessorExt, System, SystemExt};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut system = System::new_all();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Matrix-Terminal")
                .borders(Borders::ALL);
            f.render_widget(block, size);


            for cpu in system.cpus(){
                let cpu_usage = cpu.cpu_usage() as u16;
                let gauge = Gauge::default()
                    .block(Block::default().title("CPU Usage").borders(Borders::ALL))
                    .gauge_style(Style::default().fg(Color::White).bg(Color::Black).add_modifier(Modifier::ITALIC))
                    .percent(cpu_usage);
                f.render_widget(gauge, size);
            }

        })?;

        system.refresh_all();
        thread::sleep(Duration::from_secs(1));
    }

    // restore terminal
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;
    // Ok(())
}

// use sysinfo::{CpuExt, System, SystemExt};
// use std::{thread, time::Duration};
// use std::io;
//
// fn main() {
//     let mut system = System::new_all();
//     loop {
//         system.refresh_cpu();
//         for cpu in system.cpus() {
//             println!("cpu usage: {}%", cpu.cpu_usage());
//         }
//         thread::sleep(Duration::from_secs(1));
//     }
// }