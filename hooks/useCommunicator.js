import { useContext } from 'react';
import { CommunicatorContext } from '../context/CommunicatorContext';

export const useCommunicator = () => {
  const context = useContext(CommunicatorContext);
  if (!context) {
    throw new Error('useCommunicator must be used within CommunicatorProvider');
  }
  return context;
};
