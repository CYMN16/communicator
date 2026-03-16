import React, { useState } from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';

export default function EditMode() {
  const {
    planes,
    activePlaneId,
    getActivePlane,
    addCategory,
    addTerm,
    addPlane,
    deletePlane,
    updateCategoryColor,
    expandedCategoryId,
    editMode,
    toggleEditMode,
  } = useCommunicator();

  const [newCategoryName, setNewCategoryName] = useState('');
  const [newCategoryColor, setNewCategoryColor] = useState('#4CAF50');
  const [newTermName, setNewTermName] = useState('');
  const [selectedCategoryForTerm, setSelectedCategoryForTerm] = useState('');
  const [newPlaneName, setNewPlaneName] = useState('');

  const activePlane = getActivePlane();

  const handleAddCategory = (e) => {
    e.preventDefault();
    if (newCategoryName.trim()) {
      addCategory(newCategoryName, newCategoryColor);
      setNewCategoryName('');
      setNewCategoryColor('#4CAF50');
    }
  };

  const handleAddTerm = (e) => {
    e.preventDefault();
    const targetCategoryId = selectedCategoryForTerm || expandedCategoryId;
    if (newTermName.trim() && targetCategoryId) {
      addTerm(targetCategoryId, newTermName);
      setNewTermName('');
    }
  };

  const handleAddPlane = (e) => {
    e.preventDefault();
    if (newPlaneName.trim()) {
      addPlane(newPlaneName);
      setNewPlaneName('');
    }
  };

  return (
    <div className={styles.editModePanel}>
      <div className={styles.editModeHeader}>
        <h2>Edit Mode</h2>
        <button
          className={styles.closeEditButton}
          onClick={toggleEditMode}
          title="Exit edit mode"
        >
          ✕
        </button>
      </div>

      <div className={styles.editSection}>
        <h3>Add New Plane</h3>
        <form onSubmit={handleAddPlane} className={styles.editForm}>
          <input
            type="text"
            placeholder="Plane name"
            value={newPlaneName}
            onChange={(e) => setNewPlaneName(e.target.value)}
            className={styles.editInput}
          />
          <button type="submit" className={styles.editFormButton}>
            + Add Plane
          </button>
        </form>
      </div>

      <div className={styles.editSection}>
        <div className={styles.editSectionHeader}>
          <h3>Current Plane: {activePlane?.name}</h3>
          {planes.length > 1 && (
            <button
              className={styles.dangerButton}
              onClick={() => {
                if (confirm(`Delete plane "${activePlane?.name}"?`)) {
                  deletePlane(activePlaneId);
                }
              }}
            >
              Delete Plane
            </button>
          )}
        </div>
      </div>

      <div className={styles.editSection}>
        <h3>Add Category</h3>
        <form onSubmit={handleAddCategory} className={styles.editForm}>
          <input
            type="text"
            placeholder="Category name"
            value={newCategoryName}
            onChange={(e) => setNewCategoryName(e.target.value)}
            className={styles.editInput}
          />
          <div className={styles.colorPickerRow}>
            <label>Color:</label>
            <input
              type="color"
              value={newCategoryColor}
              onChange={(e) => setNewCategoryColor(e.target.value)}
              className={styles.editColorInput}
            />
            <span
              className={styles.colorPreview}
              style={{ backgroundColor: newCategoryColor }}
            ></span>
          </div>
          <button type="submit" className={styles.editFormButton}>
            + Add Category
          </button>
        </form>
      </div>

      <div className={styles.editSection}>
        <h3>Edit Category Colors</h3>
        <div className={styles.categoryColorList}>
          {activePlane?.categories.map((category) => (
            <div key={category.id} className={styles.categoryColorEditor}>
              <span className={styles.categoryLabel}>{category.name}</span>
              <input
                type="color"
                value={category.color}
                onChange={(e) => updateCategoryColor(category.id, e.target.value)}
                className={styles.editColorInput}
              />
              <span
                className={styles.colorPreview}
                style={{ backgroundColor: category.color }}
              ></span>
            </div>
          ))}
        </div>
      </div>

      <div className={styles.editSection}>
        <h3>Add Term</h3>
        <form onSubmit={handleAddTerm} className={styles.editForm}>
          <select
            value={selectedCategoryForTerm}
            onChange={(e) => setSelectedCategoryForTerm(e.target.value)}
            className={styles.editSelect}
          >
            <option value="">
              {expandedCategoryId
                ? 'Current Category'
                : 'Select a category'}
            </option>
            {activePlane?.categories.map((category) => (
              <option key={category.id} value={category.id}>
                {category.name}
              </option>
            ))}
          </select>
          <input
            type="text"
            placeholder="Term name"
            value={newTermName}
            onChange={(e) => setNewTermName(e.target.value)}
            className={styles.editInput}
          />
          <button
            type="submit"
            className={styles.editFormButton}
            disabled={!selectedCategoryForTerm && !expandedCategoryId}
          >
            + Add Term
          </button>
        </form>
      </div>

      <div className={styles.editFooter}>
        <button
          className={styles.editModeCloseButton}
          onClick={toggleEditMode}
        >
          Done Editing
        </button>
      </div>
    </div>
  );
}
