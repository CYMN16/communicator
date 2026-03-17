import React from 'react';
import styles from '../styles/Communicator.module.css';
import CategoryGrid from './CategoryGrid';
import TermGrid from './TermGrid';
import BackButton from './BackButton';
import SentenceDock from './SentenceDock';
import EditMode from './EditMode';
import { useCommunicator } from '../hooks/useCommunicator';
import { getTranslation } from '../utils/translations';

export default function CommunicatorApp() {
  const { expandedCategoryId, editMode, isHydrated, locale } = useCommunicator();

  if (!isHydrated) {
    return <div className={styles.communicatorContainer}>{getTranslation(locale, 'loading')}...</div>;
  }

  return (
    <div className={styles.communicatorContainer}>
      
      {editMode ? (
        <EditMode />
      ) : (
        <div className={styles.workspace}>
          <div
            style={{
              padding: '16px',
              background: 'rgba(255, 255, 255, 0.5)',
              borderRadius: '12px',
              marginBottom: '24px',
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <h1 className={styles.planeTitle}>
                {expandedCategoryId 
                ? getTranslation(locale, "terms") 
                : getTranslation(locale, "categories")}
              </h1>
              <button 
                onClick={() => window.dispatchEvent(new CustomEvent('focus-sentence-input'))}
                className={styles.keyboardButton}
                aria-label="Open keyboard"
                title="Open keyboard"
              >
                ⌨️
              </button>
            </div>
            <div className={styles.depthIndicator}>
              {expandedCategoryId
                ? getTranslation(locale, 'expandedView')
                : getTranslation(locale, 'selectCategory')}
            </div>
          </div>

          <div className={styles.contentArea}>
            {expandedCategoryId ? (
              <>
                <BackButton />
                <TermGrid />
              </>
            ) : (
              <CategoryGrid />
            )}
          </div>
        </div>
      )}

      <SentenceDock />
    </div>
  );
}
