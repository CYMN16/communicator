import React from 'react';
import CommunicatorApp from '../components/CommunicatorApp';
import { CommunicatorProvider } from '../context/CommunicatorContext';

export default function Home() {
  return (
    <CommunicatorProvider>
      <CommunicatorApp />
    </CommunicatorProvider>
  );
}