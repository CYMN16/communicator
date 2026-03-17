import React, { useState } from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';
import { getTranslation } from '../utils/translations';

export default function TermGrid({ editMode = false }) {
  const {
    getCategories,
    expandedCategoryId,
    addTermToCollection,
    addTerm,
    deleteTerm,
    updateTermName,
    locale,
  } = useCommunicator();
  const categories = getCategories();
  const expandedCategory = categories.find((c) => c.id === expandedCategoryId);

  const [editingId, setEditingId] = useState(null);
  const [editingName, setEditingName] = useState('');
  const [longPressTimer, setLongPressTimer] = useState(null);
  const [newTermName, setNewTermName] = useState('');

  const handleTermClick = (term) => {
    if (editMode && editingId !== term.id) {
      // Start editing if tapped in edit mode
      setEditingId(term.id);
      setEditingName(term.name);
    } else if (!editMode) {
      // Normal collection
      addTermToCollection(term, expandedCategoryId);
    }
  };

  const handleMouseDown = (termId) => {
    if (!editMode) return;
    
    const timer = setTimeout(() => {
      // Long press - show delete confirmation
      const term = expandedCategory?.children.find((t) => t.id === termId);
      if (term && confirm(`Delete term "${term.name}"?`)) {
        deleteTerm(expandedCategoryId, termId);
      }
    }, 500);
    
    setLongPressTimer(timer);
  };

  const handleMouseUp = () => {
    if (longPressTimer) {
      clearTimeout(longPressTimer);
      setLongPressTimer(null);
    }
  };

  const handleSaveEdit = (termId) => {
    if (editingName.trim()) {
      updateTermName(expandedCategoryId, termId, editingName);
    }
    setEditingId(null);
  };

  const handleAddTerm = (e) => {
    e.preventDefault();
    if (newTermName.trim()) {
      addTerm(expandedCategoryId, newTermName);
      setNewTermName('');
    }
  };

  if (!expandedCategory) return null;

  return (
    <div>
      {editMode && (
        <div style={{ marginBottom: '24px', padding: '12px', background: 'rgba(33, 150, 243, 0.1)', borderRadius: '8px' }}>
          <h3 style={{ margin: '0 0 12px 0', fontSize: '14px', fontWeight: 'bold' }}>
            + {getTranslation(locale, 'addNewTerm')}
          </h3>
          <form onSubmit={handleAddTerm} style={{ display: 'flex', gap: '8px' }}>
            <input
              type="text"
              placeholder={getTranslation(locale, 'termName')}
              value={newTermName}
              onChange={(e) => setNewTermName(e.target.value)}
              style={{
                flex: 1,
                padding: '8px 12px',
                border: '1px solid #ccc',
                borderRadius: '6px',
                fontSize: '14px',
              }}
            />
            <button
              type="submit"
              style={{
                padding: '8px 16px',
                background: '#2196F3',
                color: 'white',
                border: 'none',
                borderRadius: '6px',
                cursor: 'pointer',
                fontWeight: 'bold',
              }}
            >
              {getTranslation(locale, 'addTerm')}
            </button>
          </form>
        </div>
      )}

      <div className={styles.expandedCategory}>
        <div className={styles.termGrid}>
          {expandedCategory.children.map((term) => (
            <div key={term.id}>
              {editingId === term.id ? (
                // Inline edit mode
                <div
                  style={{
                    backgroundColor: expandedCategory.color,
                    borderRadius: '12px',
                    padding: '16px',
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '8px',
                    minHeight: '100px',
                    justifyContent: 'center',
                  }}
                >
                  <input
                    type="text"
                    value={editingName}
                    onChange={(e) => setEditingName(e.target.value)}
                    autoFocus
                    style={{
                      padding: '8px',
                      border: '2px solid white',
                      borderRadius: '6px',
                      fontSize: '16px',
                      fontWeight: 'bold',
                      textAlign: 'center',
                    }}
                  />
                  <div style={{ display: 'flex', gap: '6px' }}>
                    <button
                      onClick={() => handleSaveEdit(term.id)}
                      style={{
                        flex: 1,
                        padding: '8px',
                        background: 'white',
                        border: 'none',
                        borderRadius: '6px',
                        cursor: 'pointer',
                        fontWeight: 'bold',
                        fontSize: '12px',
                      }}
                    >
                      {getTranslation(locale, 'save')}
                    </button>
                    <button
                      onClick={() => setEditingId(null)}
                      style={{
                        flex: 1,
                        padding: '8px',
                        background: 'rgba(255, 255, 255, 0.6)',
                        border: 'none',
                        borderRadius: '6px',
                        cursor: 'pointer',
                        fontWeight: 'bold',
                        fontSize: '12px',
                      }}
                    >
                      {getTranslation(locale, 'cancel')}
                    </button>
                    <button
                      onClick={() => {
                        const confirmMsg = `${getTranslation(locale, 'deleteTermConfirm')} "${term.name}"?`;
                        if (confirm(confirmMsg)) {
                          deleteTerm(expandedCategoryId, term.id);
                          setEditingId(null);
                        }
                      }}
                      style={{
                        flex: 1,
                        padding: '8px',
                        background: '#ff6b6b',
                        color: 'white',
                        border: 'none',
                        borderRadius: '6px',
                        cursor: 'pointer',
                        fontWeight: 'bold',
                        fontSize: '12px',
                      }}
                    >
                      Delete
                    </button>
                  </div>
                </div>
              ) : (
                // Normal display mode
                <div
                  className={styles.termNode}
                  style={{
                    backgroundColor: expandedCategory.color,
                    cursor: editMode ? 'pointer' : 'pointer',
                    position: 'relative',
                  }}
                  onClick={() => handleTermClick(term)}
                  onMouseDown={() => handleMouseDown(term.id)}
                  onMouseUp={handleMouseUp}
                  onMouseLeave={handleMouseUp}
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
                    <>
                      <div
                        style={{
                          position: 'absolute',
                          top: '8px',
                          right: '8px',
                          background: 'rgba(0, 0, 0, 0.3)',
                          color: 'white',
                          borderRadius: '50%',
                          width: '24px',
                          height: '24px',
                          display: 'flex',
                          alignItems: 'center',
                          justifyContent: 'center',
                          fontSize: '12px',
                          fontWeight: 'bold',
                        }}
                      >
                        ✎
                      </div>
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          const confirmMsg = `${getTranslation(locale, 'deleteTermConfirm')} "${term.name}"?`;
                          if (confirm(confirmMsg)) {
                            deleteTerm(expandedCategoryId, term.id);
                          }
                        }}
                        style={{
                          position: 'absolute',
                          bottom: '8px',
                          left: '8px',
                          background: '#ff6b6b',
                          color: 'white',
                          border: 'none',
                          borderRadius: '50%',
                          width: '24px',
                          height: '24px',
                          display: 'flex',
                          alignItems: 'center',
                          justifyContent: 'center',
                          fontSize: '14px',
                          cursor: 'pointer',
                          padding: 0,
                          fontWeight: 'bold',
                        }}
                        aria-label={`Delete ${term.name}`}
                      >
                        🗑️
                      </button>
                    </>
                  )}
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
