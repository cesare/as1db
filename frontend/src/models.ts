export interface Class {
  id: number
  name: string
}

export interface Category {
  id: number
  name: string
}

export interface Item {
  id: number
  name: string
}

export interface ItemWithDetails {
  id: number
  name: string
  class: Class
  categories: Category[]
  materialItems: Item[]
  materialCategories: Category[]
}
