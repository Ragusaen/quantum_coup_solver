# Quantum Coup

Quantum coup is a fully deterministic version of the card game Coup.
It can be played without any cards or tokens, if the players are able
to remember the state of the game.

## Rules

The rules are the same as the original game, except the state of each
players cards are in a "superposition" of all possible cards,
until they perform an action that requires the card. One big difference
is that you cannot lie about your cards, since playing a move that
requires a card will collapse the superposition into a single card,
and you will now have that card.

There are 6 cards in the game: Duke, Assassin, Ambassador, Captain, Contessa.

Each player starts with two void cards, which can represent any card.
Each player also starts with two coins. The current player chooses to
perform an action from the following list:
- Income: Take one coin from the bank.
- Foreign Aid: Take two coins from the bank.
- Coup: Pay seven coins to the bank, and choose a player to lose a card.
- Tax: Take three coins from the bank. (Requires Duke)
- Assassinate: Lose 3 coins and choose a player to lose a card. (Requires Assassin)
- Exchange: Take two cards from the deck, choose one to keep, and put the other back. (Requires Ambassador)
- Steal: Choose a player to take two coins from. (Requires Captain)

If the action requires a card, the player must either already have that card,
or they must convert one void card into that card.

Other players can then challenge the action. The actions can be challenged
in the following ways:

- Income: Cannot be challenged.
- Foreign Aid: Block the action, no tokens awarded (requires Duke).
- Coup: Cannot be challenged.
- Tax: Cannot be challenged.
- Assassinate: Block the action, tokens are still paid (requires Contessa).
- Exchange: Cannot be challenged.
- Steal: Block the action, no tokens awarded (requires Captain or Ambassador).

Again, if a player challenges an action, they must either already have the
required card, or they must convert one void card into that card.

If a player has no cards left, they are eliminated from the game. The last
player standing wins.

At no point can a player have more than 10 coins. If a player steals from 
another player with only one coin, they only take one coin. If a player has
9 coins and steal from another player, they only take one coin.

***Note: I suggest to not play with the ambassador card, as this can cause
infinite play, where no player wins.***

## Example Game

Two players are playing, Alice and Bob. Alice is starting. We denote the
state of the game as `Alice: (card1, card2, coins), Bob: (card1, card2, coins)`.

At the beginning we have:
```
Alice: (void, void, 2), Bob: (void, void, 2)
```
Alice chooses to take income and gains one coin, Bob cannot block this.
```
Alice: (void, void, 3), Bob: (void, void, 2)
```
Bob chooses to tax, thereby collapsing one of his cards to be a duke.
```
Alice: (void, void, 3), Bob: (duke, void, 5)
```
Alice chooses to take foreign aid, Bob blocks this with his duke.
```
Alice: (void, void, 3), Bob: (duke, void, 5)
```
Bob chooses to tax again.
```
Alice: (void, void, 3), Bob: (duke, void, 8)
```
Alice chooses to assassinate Bob, thereby collapsing one of her cards to be an assassin. Bob blocks this, thereby collapsing his last card to be a contessa.
```
Alice: (assassin, void, 0), Bob: (duke, contessa, 8)
```
Bob chooses to coup Alice. Alice cannot block this, and must lose a card. She chooses to lose her assassin.
```
Alice: (dead, void, 0), Bob: (duke, contessa, 1)
```
Alice chooses to take income.
```
Alice: (dead, void, 1), Bob: (duke, contessa, 1)
```
Bob chooses to tax.
```
Alice: (dead, void, 1), Bob: (duke, contessa, 4)
```
Alice chooses to steal from Bob, thereby collapsing one of her cards to be a captain. Bob cannot block this, as both of his cards are already collapsed, and neither is Captain nor Ambassador.
```
Alice: (dead, captain, 3), Bob: (duke, contessa, 2)
```
Bob chooses to tax.
```
Alice: (dead, captain, 3), Bob: (duke, contessa, 5)
```
Alice chooses to steal from Bob again.
```
Alice: (dead, captain, 5), Bob: (duke, contessa, 3)
```
Bob cannot do anything but get coins, so he continues to tax.
```
Alice: (dead, captain, 5), Bob: (duke, contessa, 6)
```
Alice chooses to steal from Bob again.
```
Alice: (dead, captain, 7), Bob: (duke, contessa, 4)
```
Bob taxes again.
```
Alice: (dead, captain, 7), Bob: (duke, contessa, 7)
```
Alice chooses to steal from Bob again.
```
Alice: (dead, captain, 9), Bob: (duke, contessa, 5)
```
Bob taxes again.
```
Alice: (dead, captain, 9), Bob: (duke, contessa, 8)
```
Alice coups Bob. Bob must lose a card, and chooses to lose his contessa.
```
Alice: (dead, captain, 2), Bob: (duke, dead, 8)
```
Bob coups Alice. Alice must lose a card.
```
Alice: (dead, dead, 2), Bob: (duke, dead, 1)
```
Alice has no more cards left, and thus Bob has won the game.


## Solver

This program solves the entire state-space of the game. It finds
that in a two player game, the first player has a winning strategy.
