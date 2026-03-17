import React from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';
import { getTranslation } from '../utils/translations';

export default function BackButton() {
  const { collapseCategory, expandedCategoryId, locale } = useCommunicator();

  if (!expandedCategoryId) return null;

  return (
    <button
      className={styles.backButton}
      onClick={collapseCategory}
      aria-label="Go back to category selection"
    >
      {getTranslation(locale, 'back')}
    </button>
  );
}
