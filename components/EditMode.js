import React from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';
import { getTranslation } from '../utils/translations';
import CategoryGrid from './CategoryGrid';
import TermGrid from './TermGrid';
import BackButton from './BackButton';
import LocaleSwitch from './LocaleSwitch';

export default function EditMode() {
  const { expandedCategoryId, toggleEditMode, locale, editMode } = useCommunicator();

  return (
    <div className={styles.workspace}>
      <div
        style={{
          padding: '16px',
          background: 'rgba(255, 255, 255, 0.5)',
          borderRadius: '12px',
          marginBottom: '24px',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
        }}
      >
        <div>
          <h1 className={styles.planeTitle}>{getTranslation(locale, 'editMode')}</h1>
          <div className={styles.depthIndicator}>
            {expandedCategoryId
              ? getTranslation(locale, 'tapTermToEdit')
              : getTranslation(locale, 'tapCategoryToEdit')}
          </div>

        </div>
          <LocaleSwitch editMode={editMode} />
      </div>

      <div className={styles.contentArea}>
        {expandedCategoryId ? (
          <>
            <BackButton />
            <TermGrid editMode={true} />
          </>
        ) : (
          <CategoryGrid editMode={true} />
        )}
      </div>
    </div>
  );
}
