import React from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';

export default function SentenceDock() {
  const { collectedTerms, removeTerm, clearTerms, toggleEditMode, editMode } = useCommunicator();

  const handleSpeak = () => {
    if (typeof window !== 'undefined' && 'speechSynthesis' in window) {
      const text = collectedTerms.map((t) => t.name).join(' ');
      const utterance = new SpeechSynthesisUtterance(text);
      window.speechSynthesis.speak(utterance);
    }
  };

  return (
    <div className={styles.sentenceDock}>
      <div className={styles.dockContent}>
        {collectedTerms.length === 0 ? (
          <span style={{ color: '#999', fontSize: '13px' }}>
            Tap categories and terms to build your sentence...
          </span>
        ) : (
          collectedTerms.map((term, index) => (
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
          ))
        )}
      </div>

      <div className={styles.dockActions}>
        {collectedTerms.length > 0 && (
          <>
            <button
              className={styles.dockButton + ' ' + styles.clear}
              onClick={clearTerms}
              aria-label="Clear all terms"
            >
              Clear
            </button>
            <button
              className={styles.dockButton + ' ' + styles.speak}
              onClick={handleSpeak}
              aria-label="Speak sentence"
            >
              Speak
            </button>
          </>
        )}
        <button
          className={styles.dockButton + ' ' + styles.edit}
          onClick={toggleEditMode}
          aria-label="Toggle edit mode"
        >
          {editMode ? 'Done' : '✎ Edit'}
        </button>
      </div>
    </div>
  );
}
