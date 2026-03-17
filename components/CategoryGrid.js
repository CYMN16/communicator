import React, { useState } from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';
import { getTranslation } from '../utils/translations';

export default function CategoryGrid({ editMode = false }) {
  const {
    getCategories,
    expandCategory,
    addCategory,
    deleteCategory,
    updateCategoryName,
    updateCategoryColor,
    locale,
  } = useCommunicator();
  const categories = getCategories();

  const [editingId, setEditingId] = useState(null);
  const [editingName, setEditingName] = useState('');
  const [editingColor, setEditingColor] = useState('');
  const [longPressTimer, setLongPressTimer] = useState(null);
  const [newCategoryName, setNewCategoryName] = useState('');
  const [newCategoryColor, setNewCategoryColor] = useState('#4CAF50');

  const handleCategoryClick = (categoryId) => {
    if (editMode && editingId !== categoryId) {
      // Start editing if tapped in edit mode
      const category = categories.find((c) => c.id === categoryId);
      setEditingId(categoryId);
      setEditingName(category.name);
      setEditingColor(category.color);
    } else if (!editMode) {
      // Normal navigation
      expandCategory(categoryId);
    }
  };

  const handleMouseDown = (categoryId) => {
    if (!editMode) return;
    
    const timer = setTimeout(() => {
      // Long press - show delete confirmation
      const category = categories.find((c) => c.id === categoryId);
      if (confirm(`Delete category "${category.name}"?`)) {
        deleteCategory(categoryId);
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

  const handleSaveEdit = (categoryId) => {
    if (editingName.trim()) {
      updateCategoryName(categoryId, editingName);
    }
    if (editingColor) {
      updateCategoryColor(categoryId, editingColor);
    }
    setEditingId(null);
  };

  const handleAddCategory = (e) => {
    e.preventDefault();
    if (newCategoryName.trim()) {
      addCategory(newCategoryName, newCategoryColor);
      setNewCategoryName('');
      setNewCategoryColor('#4CAF50');
    }
  };

  return (
    <div>
      {editMode && (
        <div style={{ marginBottom: '24px', padding: '12px', background: 'rgba(76, 175, 80, 0.1)', borderRadius: '8px' }}>
          <h3 style={{ margin: '0 0 12px 0', fontSize: '14px', fontWeight: 'bold' }}>
            + {getTranslation(locale, 'addNewCategory')}
          </h3>
          <form onSubmit={handleAddCategory} style={{ display: 'flex', gap: '8px', flexWrap: 'wrap', alignItems: 'center' }}>
            <input
              type="text"
              placeholder={getTranslation(locale, 'categoryName')}
              value={newCategoryName}
              onChange={(e) => setNewCategoryName(e.target.value)}
              style={{
                flex: 1,
                minWidth: '150px',
                padding: '8px 12px',
                border: '1px solid #ccc',
                borderRadius: '6px',
                fontSize: '14px',
              }}
            />
            <div style={{ display: 'flex', gap: '4px', alignItems: 'center' }}>
              <input
                type="color"
                value={newCategoryColor}
                onChange={(e) => setNewCategoryColor(e.target.value)}
                style={{
                  width: '44px',
                  height: '44px',
                  padding: '0',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: 'pointer',
                }}
              />
              <button
                type="submit"
                style={{
                  padding: '8px 16px',
                  background: '#4CAF50',
                  color: 'white',
                  border: 'none',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontWeight: 'bold',
                }}
              >
                {getTranslation(locale, 'addCategory')}
              </button>
            </div>
          </form>
        </div>
      )}

      <div className={styles.categoryGrid}>
        {categories.map((category) => (
          <div key={category.id}>
            {editingId === category.id ? (
              // Inline edit mode with color picker
              <div
                style={{
                  backgroundColor: editingColor,
                  borderRadius: '12px',
                  padding: '16px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '8px',
                  minHeight: '160px',
                  justifyContent: 'flex-start',
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
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                  <label style={{ color: 'white', fontWeight: 'bold', fontSize: '12px', margin: 0 }}>
                    {getTranslation(locale, 'color')}:
                  </label>
                  <input
                    type="color"
                    value={editingColor}
                    onChange={(e) => setEditingColor(e.target.value)}
                    style={{
                      width: '40px',
                      height: '40px',
                      padding: '0',
                      border: '2px solid white',
                      borderRadius: '6px',
                      cursor: 'pointer',
                    }}
                  />
                </div>
                <div style={{ display: 'flex', gap: '6px' }}>
                  <button
                    onClick={() => handleSaveEdit(category.id)}
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
                      const confirmMsg = `${getTranslation(locale, 'deleteCategoryConfirm')} "${category.name}" ${getTranslation(locale, 'deleteAllTerms')}`;
                      if (confirm(confirmMsg)) {
                        deleteCategory(category.id);
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
                  {getTranslation(locale, 'delete')}
                  </button>
                </div>
              </div>
            ) : (
              // Normal display mode
              <div
                className={styles.categoryBubble}
                style={{
                  backgroundColor: category.color,
                  cursor: editMode ? 'pointer' : 'pointer',
                  position: 'relative',
                }}
                onClick={() => handleCategoryClick(category.id)}
                onMouseDown={() => handleMouseDown(category.id)}
                onMouseUp={handleMouseUp}
                onMouseLeave={handleMouseUp}
                role="button"
                tabIndex={0}
                aria-label={`Select ${category.name}`}
                onKeyPress={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    handleCategoryClick(category.id);
                  }
                }}
              >
                <div className={styles.categoryName}>{category.name}</div>
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
                        const confirmMsg = `${getTranslation(locale, 'deleteCategoryConfirm')} "${category.name}" ${getTranslation(locale, 'deleteAllTerms')}`;
                        if (confirm(confirmMsg)) {
                          deleteCategory(category.id);
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
                      aria-label={`Delete ${category.name}`}
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
  );
}
