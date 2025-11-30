load "./lib/datasource.rb"
load "./lib/graphdb.rb"

ds = DataSource.new
classes = ds.classes
items_with_class = ds.items_with_class
categories = ds.categories
item_categories = ds.item_categories
material_items = ds.material_items
material_categories = ds.material_categories

GraphDB.new.session do |s|
  s.run("match (n) detach delete n")

  classes.each do |c|
    s.run("create (:Class {name: $name})", name: c.name)
  end

  items_with_class.each do |item, clazz|
    statement = <<~CYPHER
      match (c:Class {name: $class_name})
      create
        (i:Item {name: $item_name}),
        (i) -[:BELONGS_TO]-> (c)
    CYPHER
    s.run(statement, class_name: clazz.name, item_name: item.name)
  end

  categories.each do |category|
    s.run("create (:Category {name: $name})", name: category.name)
  end

  item_categories.each do |ic|
    item, category = [ic.item, ic.category]
    statement = <<~CYPHER
      match
        (i:Item {name: $item_name}),
        (c:Category {name: $category_name})
      create
        (i) -[:BELONGS_TO]-> (c)
    CYPHER
    s.run(statement, item_name: item.name, category_name: category.name)
  end

  material_items.each do |mi|
    item, material = [mi.item, mi.material]
    statement = <<~CYPHER
      match
        (i:Item {name: $item_name}),
        (m:Item {name: $material_name})
      create
        (i) -[:REQUIRES]-> (m),
        (m) -[:MATERIAL_OF]-> (i)
    CYPHER
    s.run(statement, item_name: item.name, material_name: material.name)
  end

  material_categories.each do |mc|
    item, category = [mc.item, mc.category]
    statement = <<~CYPHER
      match
        (i:Item {name: $item_name}),
        (c:Category {name: $category_name}) <-[:BELONGS_TO]- (m:Item)
      create
        (i) -[:REQUIRES]-> (c),
        (m) -[:MATERIAL_OF {as: $category_name}]-> (i)
    CYPHER
    s.run(statement, item_name: item.name, category_name: category.name)
  end
end
