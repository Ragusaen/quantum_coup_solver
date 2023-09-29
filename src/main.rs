use std::cmp::{min};
use std::collections::{HashMap};
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash};
use std::io::{BufRead, stdin};

#[derive(Clone, PartialEq, Eq, PartialOrd, Hash, Debug)]
enum Card {
    Dead, // Influence lost
    Void, // Undetermined
    Ambassador,
    Assassin,
    Captain,
    Contessa,
    Duke
}

type PlayerId = u8;
#[derive(Clone, PartialEq, Eq, Hash)]
struct Player {
    id: u8,
    card1: Card,
    card2: Card,
    coins: u8
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.id)
            .field(&self.card1)
            .field(&self.card2)
            .field(&self.coins)
            .finish()
    }
}

impl Player {
    fn has_card(&self, card: &Card) -> bool {
        self.card1 == *card || self.card2 == *card || self.card1 == Card::Void || self.card2 == Card::Void
    }

    fn claim_card(&mut self, card: Card) {
        if !(self.card1 == card || self.card2 == card) {
            if self.card1 == Card::Void {
                self.card1 = card;
            } else if self.card2 == Card::Void {
                self.card2 = card;
            } else {
                panic!("Player has no void cards");
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Move {
    Income,
    ForeignAid,
    Coup(PlayerId),
    Tax,
    Assassinate(PlayerId),
    Exchange,
    Steal(PlayerId)
}

impl Move {
    fn is_challengable(&self) -> bool {
        match self {
            Move::Income => false,
            Move::ForeignAid => true,
            Move::Coup(_) => false,
            Move::Tax => false,
            Move::Assassinate(_) => true,
            Move::Exchange => false,
            Move::Steal(_) => true
        }
    }
}

enum ChallengeMove {
    BlockForeignAid,
    BlockAssassination,
    BlockSteal
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Phase {
    Move, // Current player chooses an action
    Challenge(Move), // Other players can challenge the action
    LoseInfluence(PlayerId), // This player must loose influence
    EndTurn, // Current player's turn is over
}


#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    players: Vec<Player>,
    turn: PlayerId,
    phase: Phase,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.players)
            .field(&self.turn)
            .field(&self.phase)
            .finish()
    }
}

impl State {
    fn canonize(&self) -> State {
        let mut new_state = self.clone();
        for i in 0..new_state.players.len() {
            if new_state.players[i].card2 < new_state.players[i].card2 {
                let tmp = new_state.players[i].card1.clone();
                new_state.players[i].card1 = new_state.players[i].card2.clone();
                new_state.players[i].card2 = tmp;
            }
        }
        new_state
    }
}

const MAX_COINS: u8 = 10;
const ASSASSINATE_COST: u8 = 3;
const COUP_COST: u8 = 7;
const INITIAL_COINS: u8 = 2;

fn move_successors(state: &State) -> Vec<State> {
    let player = &state.players[state.turn as usize];

    let mut successors: Vec<State> = vec![];

    // Income
    if player.coins < MAX_COINS {
        let mut new_state = state.clone();
        new_state.phase = Phase::EndTurn;
        new_state.players[state.turn as usize].coins += 1;
        successors.push(new_state);
    }

    // Foreign Aid
    if player.coins < MAX_COINS {
        let mut new_state = state.clone();
        new_state.phase = Phase::Challenge(Move::ForeignAid);
        successors.push(new_state);
    }

    // Coup
    if player.coins >= COUP_COST {
        for i in 0..state.players.len() {
            if i != state.turn as usize {
                let mut new_state = state.clone();
                new_state.players[state.turn as usize].coins -= COUP_COST;
                new_state.phase = Phase::LoseInfluence(i as PlayerId);
                successors.push(new_state);
            }
        }
    }

    // Tax
    if player.has_card(&Card::Duke) {
        let mut new_state = state.clone();
        new_state.players[state.turn as usize].coins = min(new_state.players[state.turn as usize].coins + 3, MAX_COINS);
        new_state.players[state.turn as usize].claim_card(Card::Duke);
        new_state.phase = Phase::EndTurn;
        successors.push(new_state);
    }

    // Assassinate
    if player.has_card(&Card::Assassin) && player.coins >= ASSASSINATE_COST {
        for i in 0..state.players.len() {
            if i != state.turn as usize {
                let mut new_state = state.clone();
                new_state.players[state.turn as usize].coins -= ASSASSINATE_COST;
                new_state.players[state.turn as usize].claim_card(Card::Assassin);
                new_state.phase = Phase::Challenge(Move::Assassinate(i as PlayerId));
                successors.push(new_state);
            }
        }
    }

    // Exchange
    // if player.has_card(&Card::Ambassador) {
    //     let mut new_state = state.clone();
    //     if player.card1 != Card::Dead {
    //         new_state.players[state.turn as usize].card1 = Card::Void;
    //     }
    //     if player.card2 != Card::Dead {
    //         new_state.players[state.turn as usize].card2 = Card::Void;
    //     }
    //     new_state.phase = Phase::EndTurn;
    //     successors.push(new_state);
    // }

    // Steal
    if player.has_card(&Card::Captain) {
        for i in 0..state.players.len() {
            if i != state.turn as usize {
                let mut new_state = state.clone();
                new_state.players[state.turn as usize].claim_card(Card::Captain);
                new_state.phase = Phase::Challenge(Move::Steal(i as PlayerId));
                successors.push(new_state);
            }
        }
    }

    successors
}

fn challenge_successors(state: &State) -> Vec<State> {
    match &state.phase {
        Phase::Challenge(the_move) => {
            let mut successors: Vec<State> = vec![];

            // One player challenges
            for i in 0..state.players.len() {
                if i != state.turn as usize {
                    match the_move {
                        Move::ForeignAid => {
                            if state.players[i].has_card(&Card::Duke) {
                                let mut new_state = state.clone();
                                new_state.players[i].claim_card(Card::Duke);
                                new_state.phase = Phase::EndTurn;
                                successors.push(new_state);
                            }
                        }
                        Move::Assassinate(_) => {
                            if state.players[i].has_card(&Card::Contessa) {
                                let mut new_state = state.clone();
                                new_state.players[i].claim_card(Card::Contessa);
                                new_state.phase = Phase::EndTurn;
                                successors.push(new_state);
                            }
                        }
                        Move::Steal(_) => {
                            for block_card in [Card::Captain, Card::Ambassador] {
                                if state.players[i].has_card(&block_card) {
                                    let mut new_state = state.clone();
                                    new_state.players[i].claim_card(block_card);
                                    new_state.phase = Phase::EndTurn;
                                    successors.push(new_state);
                                }
                            }
                        }
                        _ => panic!("Not a challengeable move")
                    }
                }
            }

            // No one challenges
            let mut new_state = state.clone();
            match the_move {
                Move::ForeignAid => {
                    new_state.players[state.turn as usize].coins = min(new_state.players[state.turn as usize].coins + 2, MAX_COINS);
                    new_state.phase = Phase::EndTurn;
                }
                Move::Assassinate(target) => {
                    new_state.phase = Phase::LoseInfluence(*target);
                }
                Move::Steal(target) => {
                    let amount = min(2, min(MAX_COINS - new_state.players[state.turn as usize].coins, new_state.players[*target as usize].coins));
                    new_state.players[state.turn as usize].coins += amount;
                    new_state.players[*target as usize].coins -= amount;
                    new_state.phase = Phase::EndTurn;
                }
                _ => panic!("Not a challengeable move")
            }
            successors.push(new_state);

            successors
        }
        _ => panic!("Not a challenge phase")
    }
}

fn lose_influence_successors(state: &State) -> Vec<State> {
    match state.phase {
        Phase::LoseInfluence(target) => {
            let mut successors: Vec<State> = vec![];
            if state.players[target as usize].card1 != Card::Dead {
                let mut new_state = state.clone();
                new_state.players.iter_mut().find(|p| p.id == target).unwrap().card1 = Card::Dead;
                new_state.phase = Phase::EndTurn;
                successors.push(new_state);
            }
            if state.players[target as usize].card2 != Card::Dead {
                let mut new_state = state.clone();
                new_state.players.iter_mut().find(|p| p.id == target).unwrap().card2 = Card::Dead;
                new_state.phase = Phase::EndTurn;
                successors.push(new_state);
            }

            successors
        },
        _ => panic!("Not a lose influence phase")
    }
}


fn successors(state: &State) -> Vec<State> {
    match state.phase {
        Phase::Move => move_successors(state),
        Phase::Challenge(_) => challenge_successors(state),
        Phase::LoseInfluence(_) => lose_influence_successors(state),
        Phase::EndTurn => {
            let mut new_state = state.clone();
            new_state.players = new_state.players.iter().filter(|p| p.card1 != Card::Dead || p.card2 != Card::Dead).map(|p| p.clone()).collect();
            new_state.turn = (new_state.turn + 1) % new_state.players.len() as PlayerId;
            new_state.phase = Phase::Move;
            vec![new_state.canonize()]
        }
    }
}

fn iff(a: bool, b: bool) -> bool {
    (!a && !b) || (a && b)
}

const CONTROLLER: PlayerId = 0;

fn recursive_solve(state: State, winner: &mut HashMap<State, bool>, strategy: &mut HashMap<State, State>) {
    if winner.len() % 1000 == 0 {
        println!("States: {:?}", winner.len());
    }
    if winner.contains_key(&state) {
        return;
    }
    // println!("Solving {:?}", state);

    if state.players.len() == 1 {
        // println!("Win state {:?}, state: {:?}", turn_player == CONTROLLER, state);
        let is_win = state.players[0].id == CONTROLLER;
        winner.insert(state, is_win);
        return;
    }

    // Label this state as a loss, because we want a minimal fixed point
    winner.insert(state.clone(), false);

    let mut win_state: Option<State> = None;
    let mut lose_state: Option<State> = None;
    for successor in successors(&state) {
        // println!("\"{:?}\" -> \"{:?}\"", state, successor);
        recursive_solve(successor.clone(), winner, strategy);
        if winner[&successor] {
            win_state = Some(successor);
        } else {
            lose_state = Some(successor);
        }
    }

    let turn_player = state.players[state.turn as usize].id;
    let is_controller_turn = match state.phase {
        Phase::Challenge(_) => turn_player != CONTROLLER,
        _ => turn_player == CONTROLLER
    };

    if is_controller_turn {
        // println!("Win state {:?}, state: {:?}", win_state.is_some(), state);
        winner.insert(state.clone(), win_state.is_some());
        strategy.insert(state.clone(), if win_state.is_some() { win_state.unwrap()} else { lose_state.unwrap() });
    } else {
        // println!("Win state {:?}, state: {:?}", lose_state.is_none(), state);
        winner.insert(state.clone(), lose_state.is_none());
        strategy.insert(state.clone(), if lose_state.is_some() { lose_state.unwrap() } else { win_state.unwrap() });
    }
    // println!("\"{:?}\" [shape={:?}, color={:?}]", state, if is_controller_turn { "box" } else { "ellipse" }, if winner[&state] { "green" } else { "red" });
}

// Implements the above function without recursion
fn non_recursive_solve(initial_state: State, winner: &mut HashMap<State, i32>, strategy: &mut HashMap<State, State>) {
    enum Action { Expand, Finish }
    let mut stack = vec![(initial_state, Action::Expand)];

    while !stack.is_empty() {
        let (state, action) = stack.pop().unwrap();
        match action {
            Action::Expand => {
                if winner.len() % 1000 == 0 {
                    println!("States: {:?}", winner.len());
                }
                if winner.contains_key(&state) {
                    continue;
                }

                if state.players.len() == 1 {
                    // println!("Win state {:?}, state: {:?}", turn_player == CONTROLLER, state);
                    let is_win = state.players[0].id == CONTROLLER;
                    winner.insert(state, if is_win { 1 } else { -1 });
                    continue;
                }

                // Label this state as a loss, because we want a minimal fixed point
                winner.insert(state.clone(), 0);

                stack.push((state.clone(), Action::Finish));
                for successor in successors(&state) {
                    // println!("\"{:?}\" -> \"{:?}\"", state, successor);
                    if !winner.contains_key(&successor) {
                        stack.push((successor.clone(), Action::Expand));
                    }
                }
            }
            Action::Finish => {
                let mut win_state: Option<State> = None;
                let mut lose_state: Option<State> = None;
                for successor in successors(&state) {
                    // println!("\"{:?}\" -> \"{:?}\"", state, successor);
                    if winner[&successor] > 0 {
                        if win_state.is_none() || (win_state.is_some() && winner[&successor] < winner[win_state.as_ref().unwrap()]) {
                            win_state = Some(successor);
                        }
                    } else {
                        if lose_state.is_none() || (lose_state.is_some() && winner[&successor] < winner[lose_state.as_ref().unwrap()]) {
                            lose_state = Some(successor);
                        }
                    }
                }


                let turn_player = state.players[state.turn as usize].id;
                let is_controller_turn = match state.phase {
                    Phase::Challenge(_) => turn_player != CONTROLLER,
                    Phase::LoseInfluence(p) => CONTROLLER == p,
                    _ => turn_player == CONTROLLER
                };

                if is_controller_turn {
                    // println!("Win state {:?}, state: {:?}", win_state.is_some(), state);
                    winner.insert(state.clone(), if win_state.is_some() { winner[&win_state.as_ref().unwrap()] + 1 } else { winner[&lose_state.as_ref().unwrap()] - 1 });
                    strategy.insert(state.clone(), if win_state.is_some() { win_state.unwrap() } else { lose_state.unwrap() });
                } else {
                    // println!("Win state {:?}, state: {:?}", lose_state.is_none(), state);
                    winner.insert(state.clone(), if lose_state.is_none() { winner[&win_state.as_ref().unwrap()] + 1 } else { winner[&lose_state.as_ref().unwrap()] - 1 });
                    strategy.insert(state.clone(), if lose_state.is_some() { lose_state.unwrap() } else { win_state.unwrap() });
                }
                // println!("\"{:?}\" [shape={:?}, color={:?}]", state, if is_controller_turn { "box" } else { "ellipse" }, if winner[&state] { "green" } else { "red" }); }
            }
        }
    }
}

fn main() {
    let initial_state = State {
        players: vec![
            Player {
                id: 0,
                card1: Card::Void,
                card2: Card::Void,
                coins: INITIAL_COINS
            },
            Player {
                id: 1,
                card1: Card::Void,
                card2: Card::Void,
                coins: INITIAL_COINS
            }
        ],
        turn: 0,
        phase: Phase::Move
    };

    println!("Num states: {:?}", MAX_COINS.pow(2) as i32 * 7_i32.pow(2 * 2) * (7 + 3));

    let mut winner: HashMap<State, i32> = HashMap::new();
    let mut strategy: HashMap<State, State> = HashMap::new();
    // println!("digraph {{");
    non_recursive_solve(initial_state.clone(), &mut winner, &mut strategy);
    // println!("}}");

    let mut trace = vec![initial_state.clone()];
    while strategy.contains_key(trace.last().unwrap()) {
        let next_state = &strategy[trace.last().unwrap()];
        if trace.contains(next_state) {
            trace.push(next_state.clone());
            break;
        }
        trace.push(next_state.clone());
    }

    println!("Winner {:?}", winner[&initial_state]);
    println!("Strategy {:?}", trace);


    let mut current_state = initial_state.clone();

    while current_state.players.len() > 1 {
        println!("Current state: {:?}", current_state);

        let turn_player = current_state.players[current_state.turn as usize].id;
        let is_controller_turn = !match current_state.phase {
            Phase::Challenge(_) => turn_player != CONTROLLER,
            Phase::LoseInfluence(p) => CONTROLLER == p,
            _ => turn_player == CONTROLLER
        };

        if is_controller_turn {
            let successors = successors(&current_state);
            let i = if successors.len() == 1 {
                0
            } else {
                for (i, successor) in successors.iter().enumerate() {
                    println!("{}: {:?}", i, successor);
                }

                let mut buf = String::new();
                stdin().lock().read_line(&mut buf).expect("Could not read from stdin");
                buf.trim().parse::<usize>().expect("Not an int!")
            };
            current_state = successors[i].clone();
        } else {
            current_state = strategy[&current_state].clone();
        }
    }

}
