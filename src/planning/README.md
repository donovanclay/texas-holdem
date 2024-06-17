## TODO:
1. Check logic about aces in straights (whether it counts as 1 or 14)

# Websocket Messages
* ## `Handshake`: A message to ask the server for a unique client_id. This id is used to identify them and authenticate future messages.
    ### Sender: Client
    ### Receiver: Server
    ### Contents:

[//]: # (    * ### client_id: u128)

* ## `HandshakeOk`: A message to respond to the client's `Handshake` message. This includes their assigned client_id.
    ### Sender: Server
    ### Receiver: Server
    ### Contents:
    * ### client_id: u128

* ## `StartNewTable`: A message to make a new poker table/lobby.
  ### This automatically adds the player to the new table.
  ### Sender: Client
  ### Receiver: Server
  ### Contents:
  * ### client_id: u128

* ## `StartNewTableOk`: A message to confirm the creation of a new poker table/lobby.
  ### Sender: Server
  ### Receiver: Client
  ### Contents:

[//]: # (  * ### client_id: u128)
  * ### table_id: u128

* ## `QueryTables`: A message to learn about the open tables.
  ### Sender: Client
  ### Receiver: Client
  ### Contents:
  * ### client_id: u128


* ## `TablesInfo`: A message to learn about the open tables.
  ### Sender: Server
  ### Receiver: Client
  ### Contents:

[//]: # (  * ### client_id: u128)


* ## `JoinTable`: A message to join a poker table/lobby.
  ### Sender: Client
  ### Receiver: Server
  ### Contents:
  * ### client_id: u128
  * ### table_id: u128

* ## `JoinTableOk`: A message to confirm the player joined a poker table/lobby.
  ### Sender: Server
  ### Receiver: Client
  ### Contents:
  * ### client_id: u128
  * ### table_id: u128

* ## `StartGame`: A message to start a game at a table. The leader (the person who created the table) must be the sender.
  ### Sender: Client
  ### Receiver: Server
  ### Contents:
  * ### client_id: u128
  * ### table_id: u128

* ## `GameStarted`: A message to indicate a game has started. This is sent to everyone in the game.
  ### Sender: Server
  ### Receiver: Client
  ### Contents:
  * ### table_id: u128


# Possible hands

```txt
.
├── flush
│   ├── royal flush
│   └── flush
├── straight
│   ├── straight flush
│   └── straight
├── pair
│   ├── three of a kind
│   │   ├── four of a kind
│   │   ├── full house
│   │   └── three of a kind
│   ├── two pair
│   └── pair
└── high card
```

## Ranked Poker Hands
1. Royal Flush
2. Straight Flush
3. Four of a Kind
4. Full House
5. Flush
6. Straight
7. Three of a Kind
8. Two Pair
9. Pair
10. High Card

### Boolean Check Hand Types
1. Royal Flush
2. Straight Flush
3. Flush
4. Straight

### Non-Boolean Check Hand Types (in order of ranking)
1. Four of a Kind
2. Full House
3. Three of a Kind
4. Two Pair
5. Pair
6. High Card

## Highest Possible "Remaining Scores" and the hand
* Highcard: 6188 (14H, 13C, 12H, 11S, 9H)
* Pair: 3848 (14H, 14C, 12D, 11D, 13C)
* Two Pair: 624 (14H, 14D, 13C, 13S, 12C)
* Three of a Kind: 1976 (14D, 14C, 14H, 13C, 12C)
* Straight: 9880 (13C, 12D, 11D, 10D, 14D)
* Flush: 9828 (13C, 12C, 11C, 9C, 14C)
* Full House: 10764 (13C, 13D, 14H, 14D, 14S)
* Four of a Kind: 676 (13C, 14C, 14H, 14D, 14S)
* Straight Flush: 9100 (9C, 10C, 11C, 12C, 13C)
