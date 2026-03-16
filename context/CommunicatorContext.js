import React, { createContext, useState, useCallback, useEffect } from 'react';

export const CommunicatorContext = createContext();

const DEFAULT_PLANES = [
  {
    id: 'home',
    name: 'Home',
    categories: [
      {
        id: 'actions',
        name: 'Actions',
        color: '#4CAF50',
        children: [
          { id: 'action_1', name: 'Play', type: 'term' },
          { id: 'action_2', name: 'Stop', type: 'term' },
          { id: 'action_3', name: 'Help', type: 'term' },
        ],
      },
      {
        id: 'people',
        name: 'People',
        color: '#2196F3',
        children: [
          { id: 'person_1', name: 'Mom', type: 'term' },
          { id: 'person_2', name: 'Dad', type: 'term' },
          { id: 'person_3', name: 'Doctor', type: 'term' },
        ],
      },
      {
        id: 'emotions',
        name: 'Emotions',
        color: '#FF9800',
        children: [
          { id: 'emotion_1', name: 'Happy', type: 'term' },
          { id: 'emotion_2', name: 'Tired', type: 'term' },
          { id: 'emotion_3', name: 'Confused', type: 'term' },
        ],
      },
    ],
  },
  {
    id: 'hospital',
    name: 'Hospital',
    categories: [
      {
        id: 'medical',
        name: 'Medical',
        color: '#F44336',
        children: [
          { id: 'med_1', name: 'Pain', type: 'term' },
          { id: 'med_2', name: 'Temperature', type: 'term' },
          { id: 'med_3', name: 'Medicine', type: 'term' },
        ],
      },
    ],
  },
];

export const CommunicatorProvider = ({ children }) => {
  // Initialize state from localStorage or use defaults
  const [planes, setPlanes] = useState(DEFAULT_PLANES);
  const [isHydrated, setIsHydrated] = useState(false);

  const [activePlaneId, setActivePlaneId] = useState('home');
  const [navigationPath, setNavigationPath] = useState([]);
  const [collectedTerms, setCollectedTerms] = useState([]);
  const [categoryColors, setCategoryColors] = useState({});
  const [ghostMenuOpen, setGhostMenuOpen] = useState(false);
  const [ghostMenuPosition, setGhostMenuPosition] = useState({ x: 20, y: 20 });
  const [expandedCategoryId, setExpandedCategoryId] = useState(null);
  const [editMode, setEditMode] = useState(false);

  // Load data from localStorage on mount
  useEffect(() => {
    if (typeof window !== 'undefined') {
      const savedPlanes = localStorage.getItem('communicator_planes');
      if (savedPlanes) {
        try {
          setPlanes(JSON.parse(savedPlanes));
        } catch (e) {
          console.error('Failed to load saved planes:', e);
        }
      }
      setIsHydrated(true);
    }
  }, []);

  // Save data to localStorage whenever planes change
  useEffect(() => {
    if (isHydrated && typeof window !== 'undefined') {
      localStorage.setItem('communicator_planes', JSON.stringify(planes));
    }
  }, [planes, isHydrated]);

  // Get active plane
  const getActivePlane = useCallback(() => {
    return planes.find((p) => p.id === activePlaneId);
  }, [planes, activePlaneId]);

  // Switch plane
  const switchPlane = useCallback((planeId) => {
    setActivePlaneId(planeId);
    setNavigationPath([]);
    setExpandedCategoryId(null);
  }, []);

  // Expand category
  const expandCategory = useCallback((categoryId) => {
    setExpandedCategoryId(categoryId);
    setNavigationPath([categoryId]);
  }, []);

  // Collapse (go back)
  const collapseCategory = useCallback(() => {
    setExpandedCategoryId(null);
    setNavigationPath([]);
  }, []);

  // Add term to collection (for building sentences)
  const addTermToCollection = useCallback((term, categoryId) => {
    const category = planes
      .find((p) => p.id === activePlaneId)
      ?.categories.find((c) => c.id === categoryId);

    if (category) {
      setCollectedTerms((prev) => [
        ...prev,
        { ...term, categoryId, categoryColor: category.color },
      ]);
    }
  }, [planes, activePlaneId]);

  // Remove term from collection
  const removeTerm = useCallback((index) => {
    setCollectedTerms((prev) => prev.filter((_, i) => i !== index));
  }, []);

  // Clear all terms
  const clearTerms = useCallback(() => {
    setCollectedTerms([]);
  }, []);

  // Update category color
  const updateCategoryColor = useCallback((categoryId, color) => {
    setCategoryColors((prev) => ({ ...prev, [categoryId]: color }));
    setPlanes((prev) =>
      prev.map((plane) => ({
        ...plane,
        categories: plane.categories.map((cat) =>
          cat.id === categoryId ? { ...cat, color } : cat
        ),
      }))
    );
  }, []);

  // Toggle ghost menu
  const toggleGhostMenu = useCallback(() => {
    setGhostMenuOpen((prev) => !prev);
  }, []);

  // Move ghost menu
  const moveGhostMenu = useCallback((x, y) => {
    setGhostMenuPosition({ x, y });
  }, []);

  // Toggle edit mode
  const toggleEditMode = useCallback(() => {
    setEditMode((prev) => !prev);
  }, []);

  // Add new category to active plane
  const addCategory = useCallback((categoryName, color) => {
    const newCategoryId = 'cat_' + Date.now();
    setPlanes((prev) =>
      prev.map((plane) => {
        if (plane.id === activePlaneId) {
          return {
            ...plane,
            categories: [
              ...plane.categories,
              {
                id: newCategoryId,
                name: categoryName,
                color: color,
                children: [],
              },
            ],
          };
        }
        return plane;
      })
    );
  }, [activePlaneId]);

  // Add new term to category
  const addTerm = useCallback(
    (categoryId, termName) => {
      const newTermId = 'term_' + Date.now();
      setPlanes((prev) =>
        prev.map((plane) => ({
          ...plane,
          categories: plane.categories.map((cat) => {
            if (cat.id === categoryId) {
              return {
                ...cat,
                children: [
                  ...cat.children,
                  { id: newTermId, name: termName, type: 'term' },
                ],
              };
            }
            return cat;
          }),
        }))
      );
    },
    []
  );

  // Delete category
  const deleteCategory = useCallback((categoryId) => {
    setPlanes((prev) =>
      prev.map((plane) => ({
        ...plane,
        categories: plane.categories.filter((c) => c.id !== categoryId),
      }))
    );
  }, []);

  // Delete term
  const deleteTerm = useCallback((categoryId, termId) => {
    setPlanes((prev) =>
      prev.map((plane) => ({
        ...plane,
        categories: plane.categories.map((cat) => {
          if (cat.id === categoryId) {
            return {
              ...cat,
              children: cat.children.filter((t) => t.id !== termId),
            };
          }
          return cat;
        }),
      }))
    );
  }, []);

  // Add new plane
  const addPlane = useCallback((planeName) => {
    const newPlaneId = 'plane_' + Date.now();
    setPlanes((prev) => [
      ...prev,
      {
        id: newPlaneId,
        name: planeName,
        categories: [],
      },
    ]);
  }, []);

  // Delete plane
  const deletePlane = useCallback((planeId) => {
    setPlanes((prev) => prev.filter((p) => p.id !== planeId));
    if (activePlaneId === planeId) {
      setActivePlaneId(prev.length > 1 ? prev[0].id : 'home');
    }
  }, [activePlaneId]);

  const value = {
    planes,
    activePlaneId,
    navigationPath,
    collectedTerms,
    categoryColors,
    ghostMenuOpen,
    ghostMenuPosition,
    expandedCategoryId,
    editMode,
    isHydrated,
    getActivePlane,
    switchPlane,
    expandCategory,
    collapseCategory,
    addTermToCollection,
    removeTerm,
    clearTerms,
    updateCategoryColor,
    toggleGhostMenu,
    moveGhostMenu,
    toggleEditMode,
    addCategory,
    addTerm,
    deleteCategory,
    deleteTerm,
    addPlane,
    deletePlane,
  };

  return (
    <CommunicatorContext.Provider value={value}>
      {children}
    </CommunicatorContext.Provider>
  );
};
