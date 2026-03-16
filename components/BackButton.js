import React from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';

export default function BackButton() {
  const { collapseCategory, expandedCategoryId } = useCommunicator();

  if (!expandedCategoryId) return null;

  return (
    <button
      className={styles.backButton}
      onClick={collapseCategory}
      aria-label="Go back to category selection"
    >
      ← Back
    </button>
  );
}
