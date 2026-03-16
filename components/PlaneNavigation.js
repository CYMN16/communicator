import React from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';

export default function PlaneNavigation() {
  const { getActivePlane, expandedCategoryId, navigationPath } = useCommunicator();
  const activePlane = getActivePlane();

  return (
    <div className={styles.breadcrumbTrail}>
      <div className={styles.breadcrumbItem + ' ' + styles.active}>
        {activePlane?.name}
      </div>
      {navigationPath.length > 0 && (
        <>
          <span className={styles.breadcrumbSeparator}>/</span>
          <div className={styles.breadcrumbItem + ' ' + styles.active}>
            {activePlane?.categories.find((c) => c.id === expandedCategoryId)?.name}
          </div>
        </>
      )}
    </div>
  );
}
