mod wordlist;
use wordlist::get_wordlist;

use tui::{
    Terminal,
    backend::Backend,
    widgets::{Block, Borders, Paragraph, Wrap},
    layout::{Layout, Alignment, Rect, Constraint, Direction},
    text::Span,
    style::{Style, Color, Modifier}
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers}
};

use rand::{
    thread_rng,
    seq::SliceRandom
};

pub struct TypeMaster {
    wordlist : Vec<&'static str>,
    show_play : bool,
    is_playing : bool,
    word_input : String,
    cursor_pos : usize,
    char_count : usize
}

impl TypeMaster {
    pub fn new() -> Self {
        Self { wordlist: vec![], show_play: false, is_playing: false, word_input: String::new(), cursor_pos: 0, char_count : 0 }
    }

    pub fn run<B: Backend>(&mut self, terminal : &mut Terminal<B>) -> Result<(), std::io::Error>{
        loop {
            self.draw(terminal)?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => self.play(),
                    KeyCode::Backspace => {
                        if self.cursor_pos > 0 {
                            self.word_input.remove(self.cursor_pos - 1);
                            self.cursor_pos -= 1;
                        }
                    },
                    KeyCode::Delete => {
                        if self.word_input.len() > self.cursor_pos {
                            self.word_input.remove(self.cursor_pos);
                        }
                    },
                    KeyCode::Left => {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                        }
                    },
                    KeyCode::Right => {
                        if self.cursor_pos < self.word_input.len() {
                            self.cursor_pos += 1;
                        }
                    },
                    KeyCode::Char(' ') => {
                        if self.wordlist.len() > 0 && self.word_input == self.wordlist[0] {
                            self.char_count += self.word_input.len();
                            self.wordlist.remove(0);
                            self.word_input.clear();
                            self.cursor_pos = 0;
                        }
                    },
                    KeyCode::Char(c) => {
                        if (c == 'u' || c == 'U') && (key.modifiers.bits() & KeyModifiers::CONTROL.bits()) > 0 {
                            self.word_input.clear();
                            self.cursor_pos = 0;
                        } else if self.is_playing {
                            self.word_input.insert(self.cursor_pos, c);
                            self.cursor_pos += 1;
                        }
                    },
                    _ => {  }
                }
            }
        }

        Ok(())
    }

    fn play(&mut self) {
        if !self.show_play {
            self.show_play = true;
        }

        if !self.is_playing {
            self.wordlist = get_wordlist();
            self.wordlist.shuffle(&mut thread_rng());
            self.is_playing = true;
        }
    }

    fn draw<B: Backend>(&self, terminal : &mut Terminal<B>) -> Result<(), std::io::Error>{
		// colors
		let blue = Color::Rgb(0x20, 0x45, 0x90);
		let baby_blue = Color::Rgb(0x40, 0x90, 0xff);

		// elements
        let title = vec![
            Span::raw("[ "),
            Span::styled("TYPE", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("MASTER", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" ]")
        ];

		let root_block = Block::default()
			.title(title)
			.title_alignment(Alignment::Center)
			.borders(Borders::ALL)
			.border_style(Style::default().fg(baby_blue).add_modifier(Modifier::BOLD))
			.style(Style::default().bg(blue));

        let comment = Paragraph::new(Span::styled("Made by rdbo", Style::default().fg(Color::White))).alignment(Alignment::Center).wrap(Wrap { trim: true});

        let play_text_block = Block::default().style(Style::default().bg(Color::White)).borders(Borders::ALL).border_style(Style::default().fg(baby_blue).add_modifier(Modifier::BOLD));
        let play_text = Paragraph::new(Span::styled("PRESS ENTER TO PLAY", Style::default().fg(baby_blue).add_modifier(Modifier::BOLD))).alignment(Alignment::Center).wrap(Wrap{ trim: true });

		// draw
        terminal.draw(|f| {
            let size = f.size();
            let comment_area = Rect::new(size.x, size.y + 2, size.width, 2);
			let center_area = centered_rect(40, 10, size);
            let play_text_area = Rect::new(center_area.x, center_area.y + center_area.height / 2, center_area.width, center_area.height / 2);

            f.render_widget(root_block, size);
            f.render_widget(comment, comment_area);
            if !self.show_play {
    			f.render_widget(play_text_block, center_area);
                f.render_widget(play_text, play_text_area);
            } else {
                let words_box_area = centered_rect(40, 40, size);
                let words_block_area = Rect::new(words_box_area.x - 2, words_box_area.y - 2, words_box_area.width + 4, words_box_area.height + 4);
                let words_block = Block::default().style(Style::default().bg(baby_blue)).borders(Borders::ALL);
                let words = self.wordlist.join(" ");
                let words_box = Paragraph::new(Span::styled(words, Style::default().fg(Color::White).add_modifier(Modifier::BOLD))).wrap(Wrap{ trim: true });

                let input_area = Rect::new(words_block_area.x, words_block_area.height + words_block_area.y + 2, words_block_area.width, 2);
                let mut input_content = String::from("> ");
                input_content.push_str(&self.word_input);
                let input_text = Paragraph::new(Span::styled(input_content, Style::default().fg(Color::White).add_modifier(Modifier::BOLD))).wrap(Wrap{ trim: true });

                let word_count_area = Rect::new(input_area.x, input_area.y + 2, input_area.width, 1);
                let mut word_count_content = String::from("Words: ");
                word_count_content.push_str(&(self.char_count / 5).to_string());
                let word_count_text = Paragraph::new(Span::styled(word_count_content, Style::default()));
                f.render_widget(words_block, words_block_area);
                f.render_widget(words_box, words_box_area);
                f.render_widget(input_text, input_area);
                f.render_widget(word_count_text, word_count_area);
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
