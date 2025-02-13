# frozen_string_literal: true

module MyModule
  PI = 3.142

  # = Foo
  #
  # Every foo has a name, so make sure you look
  # at it and make sure you like it.
  class Foo
    # @return [String]
    attr_reader :name

    # @param name [String]
    def initialize(name)
      @name = name
    end
  end

  class Bar
    # @return [Integer]
    attr_reader :size

    # @param size [Integer]
    def initialize(size)
      @size = size
    end
  end
end
