import React from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';

export default function LocaleSwitch({ editMode = false }) {
  const { locale, setLocale } = useCommunicator();

  return (
    <div className={`${styles.localeSwitch} ${editMode ? styles.visible : ''}`}>
      <button
        onClick={() => setLocale('en')}
        className={`${styles.localeButton} ${locale === 'en' ? styles.active : ''}`}
        aria-label="English"
      >
        EN
      </button>
      <button
        onClick={() => setLocale('tr')}
        className={`${styles.localeButton} ${locale === 'tr' ? styles.active : ''}`}
        aria-label="Turkish"
      >
        TR
      </button>
    </div>
  );
}
