import React, { createContext, useState, useCallback, useEffect } from 'react';

export const CommunicatorContext = createContext();

const DEFAULT_CATEGORIES = [
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
];

export const CommunicatorProvider = ({ children }) => {
  // Initialize state from localStorage or use defaults
  const [categories, setCategories] = useState(DEFAULT_CATEGORIES);
  const [isHydrated, setIsHydrated] = useState(false);

  const [navigationPath, setNavigationPath] = useState([]);
  const [collectedTerms, setCollectedTerms] = useState([]);
  const [categoryColors, setCategoryColors] = useState({});
  const [expandedCategoryId, setExpandedCategoryId] = useState(null);
  const [editMode, setEditMode] = useState(false);
  const [locale, setLocale] = useState('en');

  // Load data from localStorage on mount
  useEffect(() => {
    if (typeof window !== 'undefined') {
      const savedCategories = localStorage.getItem('communicator_categories');
      if (savedCategories) {
        try {
          setCategories(JSON.parse(savedCategories));
        } catch (e) {
          console.error('Failed to load saved categories:', e);
        }
      }
      const savedLocale = localStorage.getItem('communicator_locale');
      if (savedLocale) {
        setLocale(savedLocale);
      }
      setIsHydrated(true);
    }
  }, []);

  // Save data to localStorage whenever categories change
  useEffect(() => {
    if (isHydrated && typeof window !== 'undefined') {
      localStorage.setItem('communicator_categories', JSON.stringify(categories));
    }
  }, [categories, isHydrated]);

  // Save locale to localStorage
  useEffect(() => {
    if (typeof window !== 'undefined') {
      localStorage.setItem('communicator_locale', locale);
    }
  }, [locale]);

  // Get all categories
  const getCategories = useCallback(() => {
    return categories;
  }, [categories]);

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
    const category = categories.find((c) => c.id === categoryId);

    if (category) {
      setCollectedTerms((prev) => [
        ...prev,
        { ...term, categoryId, categoryColor: category.color },
      ]);
    }
  }, [categories]);

  // Add manually typed term
  const addManualTerm = useCallback((text) => {
    if (!text.trim()) return;
    setCollectedTerms((prev) => [
      ...prev,
      {
        id: 'manual_' + Date.now(),
        name: text.trim(),
        type: 'term',
        categoryColor: '#666' // Default neutral color
      },
    ]);
  }, []);

  // Remove term from collection
  const removeTerm = useCallback((index) => {
    setCollectedTerms((prev) => prev.filter((_, i) => i !== index));
  }, []);

  // Remove the last term (for backspace support)
  const removeLastTerm = useCallback(() => {
    setCollectedTerms((prev) => prev.slice(0, -1));
  }, []);

  // Clear all terms
  const clearTerms = useCallback(() => {
    setCollectedTerms([]);
  }, []);

  // Update category color
  const updateCategoryColor = useCallback((categoryId, color) => {
    setCategoryColors((prev) => ({ ...prev, [categoryId]: color }));
    setCategories((prev) =>
      prev.map((cat) =>
        cat.id === categoryId ? { ...cat, color } : cat
      )
    );
  }, []);

  // Toggle edit mode
  const toggleEditMode = useCallback(() => {
    setEditMode((prev) => !prev);
  }, []);

  // Add new category
  const addCategory = useCallback((categoryName, color) => {
    const newCategoryId = 'cat_' + Date.now();
    setCategories((prev) => [
      ...prev,
      {
        id: newCategoryId,
        name: categoryName,
        color: color,
        children: [],
      },
    ]);
  }, []);

  // Add new term to category
  const addTerm = useCallback(
    (categoryId, termName) => {
      const newTermId = 'term_' + Date.now();
      setCategories((prev) =>
        prev.map((cat) => {
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
        })
      );
    },
    []
  );

  // Delete category
  const deleteCategory = useCallback((categoryId) => {
    setCategories((prev) =>
      prev.filter((c) => c.id !== categoryId)
    );
  }, []);

  // Delete term
  const deleteTerm = useCallback((categoryId, termId) => {
    setCategories((prev) =>
      prev.map((cat) => {
        if (cat.id === categoryId) {
          return {
            ...cat,
            children: cat.children.filter((t) => t.id !== termId),
          };
        }
        return cat;
      })
    );
  }, []);

  // Update category name
  const updateCategoryName = useCallback((categoryId, newName) => {
    setCategories((prev) =>
      prev.map((cat) =>
        cat.id === categoryId ? { ...cat, name: newName } : cat
      )
    );
  }, []);

  // Update term name
  const updateTermName = useCallback((categoryId, termId, newName) => {
    setCategories((prev) =>
      prev.map((cat) => {
        if (cat.id === categoryId) {
          return {
            ...cat,
            children: cat.children.map((t) =>
              t.id === termId ? { ...t, name: newName } : t
            ),
          };
        }
        return cat;
      })
    );
  }, []);

  const value = {
    categories,
    navigationPath,
    collectedTerms,
    categoryColors,
    expandedCategoryId,
    editMode,
    isHydrated,
    locale,
    getCategories,
    expandCategory,
    collapseCategory,
    addTermToCollection,
    addManualTerm,
    removeTerm,
    removeLastTerm,
    clearTerms,
    updateCategoryColor,
    updateCategoryName,
    updateTermName,
    toggleEditMode,
    addCategory,
    addTerm,
    deleteCategory,
    deleteTerm,
    setLocale,
  };

  return (
    <CommunicatorContext.Provider value={value}>
      {children}
    </CommunicatorContext.Provider>
  );
};
