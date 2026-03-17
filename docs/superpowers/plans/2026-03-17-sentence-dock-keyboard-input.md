# Sentence Dock Direct Keyboard Input Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable direct text input into the sentence dock with an iPad-compatible keyboard trigger button in the header.

**Architecture:** Add `addManualTerm` and `removeLastTerm` to `CommunicatorContext`. Add an inline `<input>` to `SentenceDock` with event handlers for Space/Enter/Backspace. Add a keyboard icon button to `CommunicatorApp` that focuses this input.

**Tech Stack:** Next.js (React), Context API, CSS Modules.

---

### Task 1: Update CommunicatorContext

**Files:**
- Modify: `context/CommunicatorContext.js`

- [ ] **Step 1: Add `addManualTerm` and `removeLastTerm` functions to the provider.**

```javascript
  // Add manually typed term
  const addManualTerm = useCallback((text) => {
    if (!text.trim()) return;
    setCollectedTerms((prev) => [
      ...prev,
      { 
        id: 'manual_' + Date.now(), 
        name: text.trim(), 
        type: 'term', 
        categoryColor: '#666' // Default neutral color
      },
    ]);
  }, []);

  // Remove the last term (for backspace support)
  const removeLastTerm = useCallback(() => {
    setCollectedTerms((prev) => prev.slice(0, -1));
  }, []);
```

- [ ] **Step 2: Include new functions in the context `value` object.**

```javascript
    addManualTerm,
    removeLastTerm,
```

- [ ] **Step 3: Commit context changes.**

```bash
git add context/CommunicatorContext.js
git commit -m "feat: add manual term management to CommunicatorContext"
```

---

### Task 2: Implement Inline Input in SentenceDock

**Files:**
- Modify: `components/SentenceDock.js`
- Modify: `styles/Communicator.module.css`

- [ ] **Step 1: Update `SentenceDock` to use `addManualTerm`, `removeLastTerm`, and `useRef` for focusing.**

```javascript
import React, { useRef, useEffect } from 'react';
// ...
export default function SentenceDock() {
  const { collectedTerms, removeTerm, clearTerms, toggleEditMode, editMode, locale, addManualTerm, removeLastTerm } = useCommunicator();
  const inputRef = useRef(null);
  const [inputValue, setInputValue] = React.useState('');

  const handleKeyDown = (e) => {
    if (e.key === ' ' || e.key === 'Enter') {
      if (inputValue.trim()) {
        e.preventDefault();
        addManualTerm(inputValue);
        setInputValue('');
      }
    } else if (e.key === 'Backspace' && inputValue === '') {
      removeLastTerm();
    }
  };

  // Handle global event for focusing from header
  useEffect(() => {
    const handleFocusInput = () => {
      if (inputRef.current) {
        inputRef.current.focus();
      }
    };
    window.addEventListener('focus-sentence-input', handleFocusInput);
    return () => window.removeEventListener('focus-sentence-input', handleFocusInput);
  }, []);
```

- [ ] **Step 2: Add the `<input>` element after the terms list in `SentenceDock.js`.**

```javascript
        {collectedTerms.map((term, index) => (
          // ... existing term mapping
        ))}
        <input
          ref={inputRef}
          type="text"
          className={styles.dockInput}
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={collectedTerms.length === 0 ? getTranslation(locale, 'tapToStart') : ''}
          aria-label="Type a word"
        />
```

- [ ] **Step 3: Add styling for `.dockInput` in `styles/Communicator.module.css`.**

```css
.dockInput {
  flex: 1;
  min-width: 100px;
  border: none;
  outline: none;
  background: transparent;
  font-size: 16px;
  padding: 8px;
  color: #333;
}
```

- [ ] **Step 4: Commit UI changes.**

```bash
git add components/SentenceDock.js styles/Communicator.module.css
git commit -m "feat: add inline text input to SentenceDock"
```

---

### Task 3: Add Keyboard Trigger to Header

**Files:**
- Modify: `components/CommunicatorApp.js`

- [ ] **Step 1: Add the keyboard button next to the title.**

```javascript
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <h1 className={styles.planeTitle}>
                {expandedCategoryId 
                ? getTranslation(locale, "terms") 
                : getTranslation(locale, "categories")}
              </h1>
              <button 
                onClick={() => window.dispatchEvent(new CustomEvent('focus-sentence-input'))}
                className={styles.keyboardButton}
                aria-label="Open keyboard"
                title="Open keyboard"
              >
                ⌨️
              </button>
            </div>
```

- [ ] **Step 2: Add styling for `.keyboardButton` in `styles/Communicator.module.css`.**

```css
.keyboardButton {
  background: #eee;
  border: 1px solid #ccc;
  border-radius: 8px;
  padding: 8px 12px;
  font-size: 20px;
  cursor: pointer;
  transition: background 0.2s;
}

.keyboardButton:active {
  background: #ddd;
}
```

- [ ] **Step 3: Commit header changes.**

```bash
git add components/CommunicatorApp.js styles/Communicator.module.css
git commit -m "feat: add keyboard trigger button to header"
```

---

### Task 4: Verification

- [ ] **Step 1: Verify typing.**
  - Focus input, type "Hello", press Space.
  - Expected: "Hello" chip appears.
- [ ] **Step 2: Verify backspace.**
  - With empty input, press Backspace.
  - Expected: Last chip is removed.
- [ ] **Step 3: Verify header button.**
  - Click the ⌨️ button.
  - Expected: Input field in dock gets focus.
