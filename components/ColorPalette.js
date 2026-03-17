import React, { useState } from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';

export default function ColorPalette() {
  const { getCategories, updateCategoryColor } = useCommunicator();
  const categories = getCategories();
  const [editingCategoryId, setEditingCategoryId] = useState(null);

  const handleColorChange = (categoryId, newColor) => {
    updateCategoryColor(categoryId, newColor);
  };

  return (
    <div className={styles.colorPickerContainer}>
      <div className={styles.colorPickerTitle}>
        🎨 Customize Category Colors
      </div>
      <div className={styles.colorPickerGrid}>
        {categories.map((category) => (
          <div
            key={category.id}
            className={styles.categoryColorItem}
          >
            <div className={styles.categoryColorName}>{category.name}</div>
            <div className={styles.colorInputWrapper}>
              <input
                type="color"
                className={styles.colorInput}
                value={category.color}
                onChange={(e) =>
                  handleColorChange(category.id, e.target.value)
                }
                aria-label={`Color for ${category.name}`}
              />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
