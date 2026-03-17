# Spec: Sentence Dock Direct Keyboard Input (iPad iOS 12 Support)

Date: 2026-03-17
Topic: Adding functionality to type directly into the sentence dock on an iPad with iOS 12.

## Problem Statement
Users need a way to add terms to the sentence dock that aren't already available as predefined categories/terms. On older devices like iPad iOS 12, this requires a clear way to bring up the system keyboard.

## Proposed Solution (Approach 1: Integrated Input Field)
Add a transparent text input field at the end of the list of "chips" in the `SentenceDock`. 
Provide a "keyboard" trigger button in the main header area (near the "Categories / Terms" heading) to programmatically focus this input, which triggers the system keyboard.

## Architecture & Data Flow

### 1. `CommunicatorContext.js`
- **Function: `addManualTerm(text)`**: 
  - Purpose: Adds a new term to the `collectedTerms` array.
  - Color: Use a default neutral color (e.g., `#666`) for manually typed terms.
- **Function: `removeLastTerm()`**: 
  - Purpose: Pops the last term from the `collectedTerms` array (useful for backspace functionality).

### 2. `CommunicatorApp.js`
- **Header Button**: Add a button with a keyboard icon (⌨️) next to the `h1` heading.
- **Interaction**: This button will trigger a focus event on the input field in the `SentenceDock`.

### 3. `SentenceDock.js`
- **Inline Input**: Place an `<input type="text">` after the `collectedTerms.map(...)`.
- **Keyboard Handlers**:
  - `onKeyDown`:
    - `Space` or `Enter`: If text is present, call `addManualTerm(text)` and clear the input.
    - `Backspace`: If the input is empty, call `removeLastTerm()`.
- **Styling**: Make the input field transparent and borderless so it blends into the dock.

## Visual Design
- The keyboard icon button will be consistent with the back button and other header elements.
- The input field will maintain the same vertical alignment as the "chips" but with no background/border until focused (or even when focused, for a seamless look).

## Testing Criteria
1. Tapping the keyboard button focuses the input.
2. Typing a word and pressing space adds a new chip to the dock.
3. Pressing backspace in an empty input removes the last chip.
4. The system keyboard correctly appears on iPad iOS 12 when the input is focused.
