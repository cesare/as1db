require "neo4j/driver"

class GraphDB
  private attr_reader :driver

  def initialize
    @driver = connect
  end

  def session(&block)
    driver.session(database: "neo4j", &block)
  end

  private

  def connect
    Neo4j::Driver::GraphDatabase.driver(
      'bolt://localhost:7687',
      Neo4j::Driver::AuthTokens.none
    )
  end
end
