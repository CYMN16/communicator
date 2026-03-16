import React, { useRef, useState } from 'react';
import styles from '../styles/Communicator.module.css';
import { useCommunicator } from '../hooks/useCommunicator';

export default function GhostMenu() {
  const {
    planes,
    activePlaneId,
    ghostMenuOpen,
    ghostMenuPosition,
    toggleGhostMenu,
    moveGhostMenu,
    switchPlane,
    toggleEditMode,
  } = useCommunicator();

  const menuRef = useRef(null);
  const [isDragging, setIsDragging] = useState(false);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });

  const handleMouseDown = (e) => {
    setIsDragging(true);
    setDragOffset({
      x: e.clientX - ghostMenuPosition.x,
      y: e.clientY - ghostMenuPosition.y,
    });
  };

  const handleMouseMove = (e) => {
    if (!isDragging) return;
    moveGhostMenu(
      e.clientX - dragOffset.x,
      e.clientY - dragOffset.y
    );
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  React.useEffect(() => {
    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
      };
    }
  }, [isDragging, dragOffset, ghostMenuPosition]);

  return (
    <>
      <button
        className={styles.ghostMenuTrigger}
        onClick={toggleGhostMenu}
        aria-label="Toggle menu"
        aria-expanded={ghostMenuOpen}
      >
        ☰
      </button>

      {ghostMenuOpen && (
        <div
          ref={menuRef}
          className={styles.ghostMenuContainer + (isDragging ? ' ' + styles.dragging : '')}
          style={{
            left: `${ghostMenuPosition.x}px`,
            top: `${ghostMenuPosition.y}px`,
          }}
          onMouseDown={handleMouseDown}
        >
          <div className={styles.ghostMenuHeader}>Planes</div>
          <div className={styles.planeSelectorSection}>
            {planes.map((plane) => (
              <button
                key={plane.id}
                className={styles.planeButton + (plane.id === activePlaneId ? ' ' + styles.active : '')}
                onClick={() => {
                  switchPlane(plane.id);
                  toggleGhostMenu();
                }}
                onMouseDown={(e) => e.stopPropagation()}
              >
                {plane.name}
              </button>
            ))}
          </div>

          <div className={styles.ghostMenuHeader} style={{ marginTop: '12px' }}>
            Settings
          </div>
          <button
            className={styles.ghostMenuItem}
            onClick={() => {
              toggleEditMode();
              toggleGhostMenu();
            }}
            onMouseDown={(e) => e.stopPropagation()}
          >
            ✎ Edit Mode
          </button>
        </div>
      )}
    </>
  );
}
