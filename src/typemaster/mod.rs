mod wordlist;
use wordlist::get_wordlist;

use tui::{
    Terminal,
    backend::Backend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Alignment, Rect, Constraint, Direction},
    text::Span,
    style::{Style, Color, Modifier}
};

use crossterm::{
    event::{self, Event, KeyCode}
};

pub struct TypeMaster {
    wordlist : Vec<&'static str>,
    is_playing : bool
}

impl TypeMaster {
    pub fn new() -> Self {
        Self { wordlist: vec![], is_playing: false }
    }

    pub fn run<B: Backend>(&mut self, terminal : &mut Terminal<B>) -> Result<(), std::io::Error>{
        loop {
            self.draw(terminal)?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Enter => self.is_playing = true,
                    _ => {  }
                }
            }
        }

        Ok(())
    }

    fn draw<B: Backend>(&self, terminal : &mut Terminal<B>) -> Result<(), std::io::Error>{
		// colors
		let blue = Color::Rgb(0x20, 0x45, 0x90);
		let baby_blue = Color::Rgb(0x40, 0x90, 0xff);

		// elements
        let title = vec![Span::raw("[ "), Span::styled("TYPE", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)), Span::styled("MASTER", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)), Span::raw(" ]")];

		let root_block = Block::default()
			.title(title)
			.title_alignment(Alignment::Center)
			.borders(Borders::ALL)
			.border_style(Style::default().fg(baby_blue).add_modifier(Modifier::BOLD))
			.style(Style::default().bg(blue));

        let comment = Paragraph::new(Span::styled("Made by rdbo", Style::default().fg(Color::White))).alignment(Alignment::Center);

        let play_text_block = Block::default().style(Style::default().bg(Color::White)).borders(Borders::ALL).border_style(Style::default().fg(baby_blue).add_modifier(Modifier::BOLD));
        let play_text = Paragraph::new(Span::styled("PRESS ENTER TO PLAY", Style::default().fg(baby_blue).add_modifier(Modifier::BOLD))).alignment(Alignment::Center);

		// draw
        terminal.draw(|f| {
            let size = f.size();
            let comment_area = Rect::new(size.x, size.y + 2, size.width, 2);
			let center_area = centered_rect(40, 10, size);
            let play_text_area = Rect::new(center_area.x, center_area.y + center_area.height / 2, center_area.width, center_area.height / 2);

            f.render_widget(root_block, size);
            f.render_widget(comment, comment_area);
            if !self.is_playing {
    			f.render_widget(play_text_block, center_area);
                f.render_widget(play_text, play_text_area);
            }
        })?;

        Ok(())
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
	let popup_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints(
			[
				Constraint::Percentage((100 - percent_y) / 2),
				Constraint::Percentage(percent_y),
				Constraint::Percentage((100 - percent_y) / 2),
			]
			.as_ref(),
		)
		.split(r);

	Layout::default()
		.direction(Direction::Horizontal)
		.constraints(
			[
				Constraint::Percentage((100 - percent_x) / 2),
				Constraint::Percentage(percent_x),
				Constraint::Percentage((100 - percent_x) / 2),
			]
			.as_ref(),
		)
		.split(popup_layout[1])[1]
}
