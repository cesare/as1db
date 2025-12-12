import type { Category, Class, Item, ItemWithDetails } from "./models";

interface ClassesResponse {
  classes: Class[],
}

export const fetchClasses = async (): Promise<ClassesResponse> => {
  const response = await fetch("http://localhost:3000/classes")
  return response.json()
}

interface CategoriesResponse {
  categories: Category[],
}

export const fetchCategories = async (): Promise<CategoriesResponse> => {
  const response = await fetch("http://localhost:3000/categories")
  return response.json()
}

interface ItemsOfClassResponse {
  class: Class,
  items: Item[],
}

export const fetchItemsOfClass = async (classId: string): Promise<ItemsOfClassResponse> => {
  const response = await fetch(`http://localhost:3000/classes/${classId}`)
  return response.json()
}

interface ItemsOfCategoryResponse {
  category: Category,
  items: Item[],
}

export const fetchItemsOfCategory = async (categoryId: string): Promise<ItemsOfCategoryResponse> => {
  const response = await fetch(`http://localhost:3000/categories/${categoryId}`)
  return response.json()
}

interface ItemWithDetailsResponse {
  item: ItemWithDetails,
}

export const fetchItemWithDetails = async (itemId: string): Promise<ItemWithDetailsResponse> => {
  const response = await fetch(`http://localhost:3000/items/${itemId}`)
  return response.json()
}
