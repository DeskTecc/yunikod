use crossterm::event::{self, Event, KeyCode};
use tui::{
    Frame,
    Terminal,
    backend::Backend,
    style::{Style, Color, Modifier},
    layout::{Layout, Constraint, Alignment},
    widgets::{Block, Borders, Paragraph}, text::{Spans, Span, Text}
};
use std::io;
use crate::{game::{self, Game}, entities::player::Player, ui::{settings,how_to_play}};
use rust_i18n::t;
rust_i18n::i18n!("locales");

fn build_title<'a>(color: Color) -> Text<'a> {
    let style = Style::default().fg(color);

    Text::from(vec![
        Spans(vec![
            Span::raw(" ___ ___               __       "),
            Span::styled(" __  __           __ ", style)
        ]),
        Spans(vec![
            Span::raw("|   |   |.--.--.-----.|__|______"),
            Span::styled("|  |/  |.-----.--|  |", style)
        ]),
        Spans(vec![
            Span::raw(" \\     / |  |  |     ||  |______"),
            Span::styled("|     < |  _  |  _  |", style)
        ]),
        Spans(vec![
            Span::raw("  |___|  |_____|__|__||__|      "),
            Span::styled("|__|\\__||_____|_____|", style)
        ]),
    ])
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, lang: &mut String) -> io::Result<()> {
    let mut list_idx = 0;
    loop {
        let color = match list_idx {
            0 => Color::Green,
            1 => Color::Yellow,
            2 => Color::Blue,
            3 => Color::Red,
            _ => Color::Blue
        };
        // draw \\
        terminal.draw(|frame| draw(frame, list_idx, color, lang.to_string()))?;

        // input handler \\
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Up => {
                    if list_idx > 0 {list_idx -= 1};
                },
                KeyCode::Down => {
                    if list_idx < 3 {list_idx += 1};
                },
                KeyCode::Enter => {
                    match list_idx {
                        0 => {terminal.clear().expect("Error on change MENU->NEW GAME"); new_game(terminal, lang)}?,
                        1 => {terminal.clear().expect("Error on change MENU->HTP"); how_to_play::run(terminal, lang)}?,
                        2 => {terminal.clear().expect("Error on change MENU->SETTINGS"); settings::run(terminal, lang)}?,
                        3 => {terminal.clear().expect("Error on finish the game"); println!("Thanks for playing :)"); std::process::exit(0)},
                        _ => {}
                    }
                },
                _ => {}
            }
        }

    }
}

fn draw<'a, B: Backend>(frame: &mut Frame<B>, list_idx: usize, color: Color, lang: String) {

    rust_i18n::set_locale(&lang); //set language
  
    let mut vchunks = Layout::default()
        .constraints([Constraint::Length(7), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());
    
    let mut blocks = Vec::new();
    for i in 1..=4 {
        vchunks[i].width = 40;
        vchunks[i].x = frame.size().width/2-20;
        if list_idx == i-1 {
            blocks.push(Block::default().borders(Borders::ALL).style(Style::default().fg(color)))
        } else {
            blocks.push(Block::default().borders(Borders::ALL))
        }
    }
    
    let para_title = Paragraph::new(build_title(color))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(para_title, vchunks[0]);
    
    let para_new_game = Paragraph::new(Span::styled(t!("main.opt.new_game"), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
        .block(blocks[0].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_new_game, vchunks[1]);

    let para_htp = Paragraph::new(Span::styled(t!("main.opt.htp"), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        .block(blocks[1].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_htp, vchunks[2]);
  
    let para_settings = Paragraph::new(Span::styled(t!("main.opt.settings"), Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        .block(blocks[2].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_settings, vchunks[3]);

    let para_exit = Paragraph::new(Span::styled(t!("main.opt.exit"), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)))
        .block(blocks[3].clone())
        .alignment(Alignment::Center);
    frame.render_widget(para_exit, vchunks[4]); 
}

fn new_game<B: Backend>(terminal: &mut Terminal<B>, lang: &mut String) -> io::Result<()>{
    let mut game = Game::new();
    game.update_chunks();
    let mut x = 0.0;
    while game.perlin().get_noise(x, 0.0) < 0.0 {
        x += 1.0
    }
    let player = Player::new(x as i64, 0);
    game::run(terminal, game, player,lang)?;
    Ok(())
}
