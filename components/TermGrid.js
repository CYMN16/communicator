import React from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';

export default function TermGrid() {
  const { getActivePlane, expandedCategoryId, addTermToCollection, editMode, deleteTerm } = useCommunicator();
  const activePlane = getActivePlane();
  const expandedCategory = activePlane?.categories.find(
    (c) => c.id === expandedCategoryId
  );

  const handleTermClick = (term) => {
    if (!editMode) {
      addTermToCollection(term, expandedCategoryId);
    }
  };

  const handleDelete = (e, termId) => {
    e.stopPropagation();
    if (confirm('Delete this term?')) {
      deleteTerm(expandedCategoryId, termId);
    }
  };

  if (!expandedCategory) return null;

  return (
    <div className={styles.expandedCategory}>
      <div className={styles.termGrid}>
        {expandedCategory.children.map((term) => (
          <div
            key={term.id}
            className={styles.termNode}
            style={{
              backgroundColor: expandedCategory.color,
              cursor: editMode ? 'default' : 'pointer',
            }}
            onClick={() => handleTermClick(term)}
            role="button"
            tabIndex={0}
            aria-label={`Select ${term.name}`}
            onKeyPress={(e) => {
              if (e.key === 'Enter' || e.key === ' ') {
                handleTermClick(term);
              }
            }}
          >
            <span>{term.name}</span>
            {editMode && (
              <button
                className={styles.deleteButton}
                onClick={(e) => handleDelete(e, term.id)}
                title="Delete term"
              >
                ×
              </button>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
