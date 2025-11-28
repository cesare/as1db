require "csv"
require "pg"

csv_file_name = ARGV[0]

Category = Struct.new(:id, :name)
Item = Struct.new(:id, :name)

db = PG.connect(
  host: "localhost",
  port: 5432,
  user: ENV["POSTGRES_USER"],
  password: ENV["POSTGRES_PASSWORD"],
)

items = db.exec("select id, name from items") do |results|
  results.map do |row|
    Item.new(id: row["id"], name: row["name"])
  end
end

CSV.open(csv_file_name, headers: true) do |csv|
  csv.each do |row|
    item_name = row["item"]
    material_name = row["material"]

    item = items.find { it.name == item_name }
    if item.nil?
      $stderr.puts "item #{item_name} not found"
      next
    end

    material = items.find { it.name == material_name }
    if material.nil?
      $stderr.puts "material #{material_name} not found"
      next
    end

    db.exec("insert into material_items (item_id, material_item_id) values ($1, $2)", [item.id, material.id])
    puts "inserted: #{item.name} -> #{material.name}"
  rescue => e
    $stderr.puts "insertion failed: #{e}"
  end
end

db.close
