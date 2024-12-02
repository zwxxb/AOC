# Advent of Code 2024 - Days 1 & 2 Solutions

## Day 1: Distance and Similarity Calculation

### Algorithm Overview
- Input: Two lists of integers, paired line by line
- Part 1: Calculate minimum total distance between sorted pairs
- Part 2: Calculate similarity score based on frequency matching

### Solution Approach
1. Part 1 - Total Distance:
   - Create two vectors: left and right from input pairs
   - Sort both vectors independently 
   - Zip sorted vectors and calculate absolute differences
   - Sum all differences for total distance

2. Part 2 - Similarity Score:
   - Create frequency map of right list numbers
   - For each number in left list:
     - Multiply by its frequency in right list
     - Sum all products for final score

### Complexity
- Time: O(n log n) due to sorting
- Space: O(n) for storing lists and hash map
- n = number of input pairs

## Day 2: Red-Nosed Reports Analysis

### Algorithm Overview
- Input: Lists of integer sequences
- Part 1: Check if sequences are strictly increasing/decreasing with constraints
- Part 2: Allow removal of one number to make sequence valid

### Solution Approach
1. Part 1 - Safety Check:
   - Validate adjacent differences (1-3 range)
   - Track if sequence is strictly increasing or decreasing
   - Sequence is safe if:
     - All differences are 1-3
     - Direction is consistent (all increasing or all decreasing)

2. Part 2 - Problem Dampener:
   - First try original safety check
   - If unsafe, try removing each number once:
     - Create new sequence without current number
     - Run safety check on modified sequence
     - If any removal creates safe sequence, count as safe

### Complexity
- Time: O(n²) in worst case for Part 2
- Space: O(n) for storing sequences
- n = length of each sequence
