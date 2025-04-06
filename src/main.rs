mod ai;
mod db;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{widgets::*, Frame, Terminal};
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Style, Stylize};

enum MenuTab {
    Health,
    Chat,
}

struct App {
    current_tab: MenuTab,
    api_key_status: String,
    api_connection_status: String,
    chat_messages: Vec<String>,
    input_buffer: String,
    chat_scroll: u16
}

impl App {
    fn new() -> Self {
        App {
            current_tab: MenuTab::Health,
            api_key_status: "Not checked".to_string(),
            api_connection_status: "Disconnected".to_string(),
            chat_messages: vec!["NERVA: Hello! How can I assist you today?".to_string()],
            input_buffer: String::new(),
            chat_scroll: 0
        }
    }

    fn check_api_health(&mut self) {
        self.api_key_status = "Valid".to_string();
        self.api_connection_status = "Connected".to_string();
    }

    fn add_message(&mut self, message: String) {
        self.chat_messages.push(message);
        if self.chat_messages.len() > 100 {
            self.chat_messages.remove(0);
        }
        self.chat_scroll = 0;
    }
}


fn main() -> io::Result<()> {
    color_eyre::install();
    let mut terminal = setup_terminal()?;
    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Tab => {
                    app.current_tab = match app.current_tab {
                        MenuTab::Health => MenuTab::Chat,
                        MenuTab::Chat => MenuTab::Health,
                    }
                }
                KeyCode::Char('c') if matches!(app.current_tab, MenuTab::Health) => {
                    app.check_api_health();
                }
                KeyCode::Char(c) if matches!(app.current_tab, MenuTab::Chat) => {
                    app.input_buffer.push(c);
                }
                KeyCode::Backspace if matches!(app.current_tab, MenuTab::Chat) => {
                    app.input_buffer.pop();
                }

                KeyCode::Up if matches!(app.current_tab, MenuTab::Chat) => {
                    if (app.chat_scroll > 0) {
                        app.chat_scroll -= 1;
                    }
                }
                KeyCode::Down if matches!(app.current_tab, MenuTab::Chat) => {
                    app.chat_scroll += 1;
                }
                KeyCode::PageUp if matches!(app.current_tab, MenuTab::Chat) => {
                    app.chat_scroll = app.chat_scroll.saturating_sub(5);
                }
                KeyCode::PageDown if matches!(app.current_tab, MenuTab::Chat) => {
                    app.chat_scroll += 5;
                }

                KeyCode::Enter if matches!(app.current_tab, MenuTab::Chat) => {
                    let user_input = app.input_buffer.clone();
                    app.chat_messages.push(format!("You: {}", user_input));
                    // Here you would normally call your AI API
                    app.chat_messages.push("NERVA: [Response from AI]".to_string());
                    app.input_buffer.clear();
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Footer
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("NERVA - Networked Embedded Responsive Virtual Assistant")
        .style(Style::new().bold())
        .alignment(Alignment::Center);
    let instruction = Paragraph::new("Press 'c' to check connection (in Health Check Tab) | Tab to switch | 'q' to quit")
        .style(Style::new().bold())
        .alignment(Alignment::Center);

    f.render_widget(title, main_layout[0]);
    f.render_widget(instruction, main_layout[3]);

    // Tabs
    let tabs = Tabs::new(vec!["Health Check", "Chat Mode"])
        .block(Block::default().borders(Borders::ALL))
        .select(match app.current_tab {
            MenuTab::Health => 0,
            MenuTab::Chat => 1,
        })
        .highlight_style(Style::new().bold().fg(Color::Yellow));
    f.render_widget(tabs, main_layout[1]);

    // Content
    match app.current_tab {
        MenuTab::Health => render_health_tab(f, app, main_layout[2]),
        MenuTab::Chat => render_chat_tab(f, app, main_layout[2]),
    }
}

fn render_health_tab(f: &mut Frame, app: &App, area: Rect) {
    let health_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    let status_block = Block::default()
        .title("API Status")
        .borders(Borders::ALL);

    let key_status = Paragraph::new(format!("API Key: {}", app.api_key_status))
        .block(status_block.clone());
    f.render_widget(key_status, health_layout[0]);

    let connection_status = Paragraph::new(format!("Connection: {}", app.api_connection_status))
        .block(status_block.clone());
    f.render_widget(connection_status, health_layout[1]);
}

fn render_chat_tab(f: &mut Frame, app: &App, area: Rect) {
    let chat_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1), // Chat history
            Constraint::Length(3), // Input
        ])
        .split(area);

    // Chat history
    let chat_block = Block::default()
        .title("Conversation (↑/↓ to scroll)")
        .borders(Borders::ALL);
    let chat_text = app.chat_messages.join("\n");
    let mut chat_history = Paragraph::new(chat_text)
        .block(chat_block)
        .scroll((app.chat_scroll, 0));
    f.render_widget(chat_history, chat_layout[0]);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    f.render_stateful_widget(
        scrollbar,
        chat_layout[0].inner(Margin { vertical: 1, horizontal: 1 }),
        &mut ScrollbarState::new(
            app.chat_messages.len().saturating_sub(1) as u16 as usize
        )
            .position(app.chat_scroll as usize),
    );

    // Input area
    let input_block = Block::default()
        .title("Your Message")
        .borders(Borders::ALL);
    let input = Paragraph::new(app.input_buffer.as_str())
        .block(input_block);
    f.render_widget(input, chat_layout[1]);
    f.set_cursor(
        chat_layout[1].x + app.input_buffer.len() as u16 + 1,
        chat_layout[1].y + 1,
    );
}
