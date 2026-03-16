import React from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';

export default function CategoryGrid() {
  const { getActivePlane, expandCategory, editMode, deleteCategory } = useCommunicator();
  const activePlane = getActivePlane();

  const handleCategoryClick = (categoryId) => {
    if (!editMode) {
      expandCategory(categoryId);
    }
  };

  const handleDelete = (e, categoryId) => {
    e.stopPropagation();
    if (confirm('Delete this category? All terms will be removed.')) {
      deleteCategory(categoryId);
    }
  };

  return (
    <div className={styles.categoryGrid}>
      {activePlane?.categories.map((category) => (
        <div
          key={category.id}
          className={styles.categoryBubble}
          style={{
            backgroundColor: category.color,
            cursor: editMode ? 'default' : 'pointer',
          }}
          onClick={() => handleCategoryClick(category.id)}
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
            <button
              className={styles.deleteButton}
              onClick={(e) => handleDelete(e, category.id)}
              title="Delete category"
            >
              ×
            </button>
          )}
        </div>
      ))}
    </div>
  );
}
