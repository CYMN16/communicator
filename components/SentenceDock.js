import React, { useRef, useEffect, useState } from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';
import { getTranslation } from '../utils/translations';

export default function SentenceDock() {
  const { 
    collectedTerms, 
    removeTerm, 
    clearTerms, 
    toggleEditMode, 
    editMode, 
    locale, 
    addManualTerm, 
    removeLastTerm 
  } = useCommunicator();
  
  const inputRef = useRef(null);
  const [inputValue, setInputValue] = useState('');

  const handleSpeak = () => {
    if (typeof window !== 'undefined' && 'speechSynthesis' in window) {
      const text = collectedTerms.map((t) => t.name).join(' ');
      const utterance = new SpeechSynthesisUtterance(text);
      window.speechSynthesis.speak(utterance);
    }
  };

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

  return (
    <div className={styles.sentenceDock}>
      {/* Sentence builder area */}
      <div className={styles.dockContent}>
        {collectedTerms.map((term, index) => (
          <div
            key={index}
            className={styles.dockTerm}
            style={{
              '--term-color': term.categoryColor || '#666',
            }}
          >
            {term.name}
            <span
              className={styles.dockTermRemove}
              onClick={() => removeTerm(index)}
              role="button"
              tabIndex={0}
              aria-label={`Remove ${term.name}`}
              onKeyPress={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  removeTerm(index);
                }
              }}
            >
              ✕
            </span>
          </div>
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
      </div>

      {/* Control buttons - separated from sentence builder */}
      <div className={styles.dockActions}>
        <button
          className={styles.dockButton + ' ' + styles.edit}
          onClick={toggleEditMode}
          aria-label="Toggle edit mode"
          title={editMode ? getTranslation(locale, 'exitEditMode') : getTranslation(locale, 'enterEditMode')}
        >
          {editMode ? getTranslation(locale, 'done') : '✎ ' + getTranslation(locale, 'edit')}
        </button>
        
        {collectedTerms.length > 0 && (
          <>
            <button
              className={styles.dockButton + ' ' + styles.clear}
              onClick={clearTerms}
              aria-label="Clear all terms"
              title={getTranslation(locale, 'clearAllWords')}
            >
              {getTranslation(locale, 'clear')}
            </button>
            <button
              className={styles.dockButton + ' ' + styles.speak}
              onClick={handleSpeak}
              aria-label="Speak sentence"
              title={getTranslation(locale, 'speakWords')}
            >
              {getTranslation(locale, 'speak')}
            </button>
          </>
        )}
      </div>
    </div>
  );
}
