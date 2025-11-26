require "csv"
require "pg"

csv_file_name = ARGV[0]

Trait = Struct.new(:id, :name)

db = PG.connect(
  host: "localhost",
  port: 5432,
  user: ENV["POSTGRES_USER"],
  password: ENV["POSTGRES_PASSWORD"],
)

traits = db.exec("select id, name from traits") do |results|
  results.map do |row|
    Trait.new(id: row["id"], name: row["name"])
  end
end

CSV.open(csv_file_name, headers: true) do |csv|
  csv.each.with_index(2) do |row, i|
    trait_name = row["product"]
    lhs_name = row["lhs"]
    rhs_name = row["rhs"]

    trait = traits.find { it.name == trait_name }
    lhs = traits.find { it.name == lhs_name }
    rhs = traits.find { it.name == rhs_name }

    if trait.nil? || lhs.nil? || rhs.nil?
      $stderr.puts "invalid row (#{i}): trait=#{trait}, lhs=#{lhs}, rhs=#{rhs}"
      next
    end

    db.exec("insert into trait_compositions (trait_id, lhs_id, rhs_id) values ($1, $2, $3)", [trait.id, lhs.id, rhs.id])
    puts "inserted #{trait.name} <- #{lhs.name}, #{rhs.name}"
  rescue => e
    $stderr.puts "insertion failed: #{e}"
  end
end

db.close
