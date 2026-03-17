export const translations = {
  en: {
    // Main UI
    addNewCategory: "Add New Category",
    addNewTerm: "Add New Term",
    categoryName: "Category Name",
    termName: "Term Name",
    selectColor: "Select Color",
    tapToStart: "Tap categories and terms to build your sentence...",
    categories: "Categories",
    terms: "Terms",
    loading: "Loading",
    expandedView: "Expanded view - Select a term",
    selectCategory: "Select a category to continue",

    // Buttons
    save: "Save",
    cancel: "Cancel",
    delete: "Delete",
    back: "← Back",
    done: "Done",
    edit: "Edit",
    speak: "Speak",
    clear: "Clear",
    
    // Edit mode
    enterEditMode: "Enter edit mode",
    exitEditMode: "Exit edit mode",
    doneEditing: "Done Editing",
    tapTermToEdit: "Tap a term to edit",
    tapCategoryToEdit: "Tap a category to edit",
    
    // Tooltips
    clearAllWords: "Clear all selected words",
    speakWords: "Speak the selected words",
    
    // Confirmations
    deleteCategoryConfirm: "Delete category",
    deleteTermConfirm: "Delete term",
    deleteAllTerms: "and all its terms?",
    
    // Edit Mode
    editMode: "Edit Mode",
    addCategory: "Add Category",
    addTerm: "Add Term",
    name: "Name",
    color: "Color",
    language: "Language",
  },
  tr: {
    // Main UI
    addNewCategory: "Yeni Kategori Ekle",
    addNewTerm: "Yeni Terim Ekle",
    categoryName: "Kategori Adı",
    termName: "Terim Adı",
    selectColor: "Renk Seç",
    tapToStart: "Cümle oluşturmak için terim seçiniz...",
    categories: "Kategoriler",
    terms: "Terimler",
    loading: "Yükleniyor",
    expandedView: "Terim seçiniz",
    selectCategory: "Bir kategori seçiniz",
    
    // Buttons
    save: "Kaydet",
    cancel: "İptal",
    delete: "Sil",
    back: "← Geri",
    done: "Bitti",
    edit: "Düzenle",
    speak: "Seslendir",
    clear: "Temizle",
    
    // Edit mode
    enterEditMode: "Düzenleme moduna gir",
    exitEditMode: "Düzenleme modundan çık",
    doneEditing: "Düzenleme Bitti",
    tapTermToEdit: "Düzenlemek için bir terime dokunun",
    tapCategoryToEdit: "Düzenlemek için bir kategoriye dokunun",

    // Tooltips
    clearAllWords: "Tüm seçilen sözcükleri temizle",
    speakWords: "Seçilen sözcükleri seslendir",
    
    // Confirmations
    deleteCategoryConfirm: "Kategoriyi",
    deleteTermConfirm: "Terimi",
    deleteAllTerms: "ve tüm terimlerini sil?",
    
    // Edit Mode
    editMode: "Düzenleme Modu",
    addCategory: "Kategori Ekle",
    addTerm: "Terim Ekle",
    name: "Ad",
    color: "Renk",
    language: "Dil",
  },
};

export const getTranslation = (locale, key) => {
  return translations[locale]?.[key] || translations.en[key] || key;
};
