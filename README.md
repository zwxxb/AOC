# Advent of Code 2024 - Days 1, 2, 3, 4 & 5 Solutions

## Day 1: Distance and Similarity Calculation

### Algorithm Overview
- **Input:** Two lists of integers, paired line by line
- **Part 1:** Calculate minimum total distance between sorted pairs
- **Part 2:** Calculate similarity score based on frequency matching

### Solution Approach
1. **Part 1 - Total Distance:**
   - Create two vectors: left and right from input pairs
   - Sort both vectors independently
   - Zip sorted vectors and calculate absolute differences
   - Sum all differences for total distance

2. **Part 2 - Similarity Score:**
   - Create frequency map of right list numbers
   - For each number in left list:
     - Multiply by its frequency in right list
     - Sum all products for final score

### Complexity
- **Time:** O(n log n) due to sorting
- **Space:** O(n) for storing lists and hash map
- **n:** Number of input pairs

## Day 2: Red-Nosed Reports Analysis

### Algorithm Overview
- **Input:** Lists of integer sequences
- **Part 1:** Check if sequences are strictly increasing/decreasing with constraints
- **Part 2:** Allow removal of one number to make sequence valid

### Solution Approach
1. **Part 1 - Safety Check:**
   - Validate adjacent differences (1-3 range)
   - Track if sequence is strictly increasing or decreasing
   - Sequence is safe if:
     - All differences are 1-3
     - Direction is consistent (all increasing or all decreasing)

2. **Part 2 - Problem Dampener:**
   - First try original safety check
   - If unsafe, try removing each number once:
     - Create new sequence without current number
     - Run safety check on modified sequence
     - If any removal creates safe sequence, count as safe

### Complexity
- **Time:** O(n²) in worst case for Part 2
- **Space:** O(n) for storing sequences
- **n:** Length of each sequence

## Day 3: Mull It Over

### Algorithm Overview
- **Input:** A corrupted memory string containing `mul(X,Y)` instructions and conditional `do()`/`don't()` commands
- **Part 1:** Identify and sum all valid `mul(X,Y)` multiplication results
- **Part 2:** Handle `do()` and `don't()` instructions to enable or disable `mul()` operations and sum only the enabled multiplications

### Solution Approach
1. **Part 1 - Summing Multiplications:**
   - Use regular expressions to find all valid `mul(X,Y)` patterns where `X` and `Y` are 1-3 digit numbers
   - Extract the numerical values of `X` and `Y`
   - Multiply `X` by `Y` for each valid instruction
   - Sum all multiplication results to obtain the final total

2. **Part 2 - Conditional Multiplications:**
   - Initialize a flag to keep track of whether `mul()` operations are enabled (starts as enabled)
   - Iterate through the corrupted memory string sequentially
   - When a `do()` instruction is encountered, enable `mul()` operations
   - When a `don't()` instruction is encountered, disable `mul()` operations
   - For each `mul(X,Y)` instruction:
     - If `mul()` is enabled, perform the multiplication and add to the sum
     - If disabled, ignore the multiplication
   - Continue processing until the end of the input string
   - Sum all enabled multiplication results for the final total

### Complexity
- **Time:** O(n) for both parts, where `n` is the length of the input string
- **Space:** O(1) for Part 1 and Part 2 (excluding input storage)
- **n:** Length of the corrupted memory string

## Day 4: XMAS Word Search

### Algorithm Overview
- **Input:** A word search grid represented as a 2D array of letters
- **Part 1:** Count all occurrences of the word "XMAS" in the grid
- **Part 2:** Identify "X-MAS" patterns where two "MAS" sequences intersect at the 'A' character, forming an 'X' shape

### Solution Approach
1. **Part 1 - Counting "XMAS" Sequences:**
   - Iterate through each cell in the grid
   - For each cell that matches the first character of "XMAS" or its reverse, check in all eight directions for the complete sequence
   - Increment a counter for each valid sequence found

### Complexity
- **Time:** O(n * m * d), where `n` is the number of rows, `m` the number of columns, and `d` the number of directions (constant 8)
- **Space:** O(n * m) for storing the grid
- **n:** Number of rows in the grid

## Day 5: Print Queue

### Algorithm Overview
- **Input:** Page ordering rules and updates
- **Part 1:** Identify which updates are already in the correct order
- **Part 2:** Correct the order of incorrectly-ordered updates

### Solution Approach
1. **Part 1 - Identifying Correctly Ordered Updates:**
   - Parse the input to separate ordering rules and updates
   - For each update, verify if it follows the ordering rules
   - Identify the middle page number of each correctly-ordered update
   - Sum the middle page numbers of the correctly-ordered updates

2. **Part 2 - Correcting Order of Updates:**
   - Identify updates that are not in the correct order
   - Use the page ordering rules to sort the pages in the correct order
   - Find the middle page number of each corrected update
   - Sum the middle page numbers of the corrected updates

### Complexity
- **Time:** O(n + m) for parsing and checking order, where `n` is the number of rules and `m` is the number of updates
- **Space:** O(n + m) for storing rules and updates
- **n:** Number of ordering rules
- **m:** Number of updates