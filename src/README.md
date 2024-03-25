## TODO:
1. Check logic about aces in straights (whether it counts as 1 or 14)

# possible hands

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
Highcard: 6188 (14H, 13C, 12H, 11S, 9H)
Pair: 3848 (14H, 14C, 12D, 11D, 13C)
Two Pair: 624 (14H, 14D, 13C, 13S, 12C)
Three of a Kind: 1976 (14D, 14C, 14H, 13C, 12C)
Straight: 9880 (13C, 12D, 11D, 10D, 14D)
Flush: 9828 (13C, 12C, 11C, 9C, 14C)
Full House: 10764 (13C, 13D, 14H, 14D, 14S)
Four of a Kind: 676 (13C, 14C, 14H, 14D, 14S)
Straight Flush: 9100 (9C, 10C, 11C, 12C, 13C)
