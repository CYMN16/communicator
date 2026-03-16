import React from 'react';
import styles from '../styles/Communicator.module.css';
import PlaneNavigation from './PlaneNavigation';
import CategoryGrid from './CategoryGrid';
import TermGrid from './TermGrid';
import BackButton from './BackButton';
import SentenceDock from './SentenceDock';
import GhostMenu from './GhostMenu';
import EditMode from './EditMode';
import { useCommunicator } from '../hooks/useCommunicator';

export default function CommunicatorApp() {
  const { getActivePlane, expandedCategoryId, editMode, isHydrated } = useCommunicator();
  const activePlane = getActivePlane();

  if (!isHydrated) {
    return <div className={styles.communicatorContainer}>Loading...</div>;
  }

  if (editMode) {
    return (
      <div className={styles.communicatorContainer}>
        <GhostMenu />
        <EditMode />
        <SentenceDock />
      </div>
    );
  }

  return (
    <div className={styles.communicatorContainer}>
      <GhostMenu />

      <div className={styles.workspace}>
        <PlaneNavigation />

        <div
          style={{
            padding: '16px',
            background: 'rgba(255, 255, 255, 0.5)',
            borderRadius: '12px',
            marginBottom: '24px',
          }}
        >
          <h1 className={styles.planeTitle}>{activePlane?.name}</h1>
          <div className={styles.depthIndicator}>
            {expandedCategoryId
              ? 'Expanded view - Select a term'
              : 'Select a category to continue'}
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

      <SentenceDock />
    </div>
  );
}
