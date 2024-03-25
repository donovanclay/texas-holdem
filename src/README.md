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
