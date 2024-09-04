# This is free and unencumbered software released into the public domain.

require 'open3'

##
# A simple interface to the `codify` program.
module Codify
  PROGRAM = (ENV['CODIFY'] || 'codify').freeze

  ##
  # Check whether the `codify` program is available.
  #
  # @return [Boolean]
  def self.available?
    !!self.version
  end

  ##
  # Returns the version of the `codify` program, if available.
  #
  # @return [String]
  def self.version
    self.execute('--version').split(' ').last rescue nil
  end

  ##
  # Executes a `codify convert` command and returns the output.
  # Raises an exception if the command fails.
  #
  # @param [String, #to_s] source_type
  # @param [String, #to_s] target_language
  # @return [String]
  # @raise [Error::NotAvailable]
  # @raise [Error::UnexpectedExit]
  def self.convert(source_type, target_language = :ruby)
    source_type = source_type.join(':') if source_type.is_a?(Array)
    self.execute('convert', source_type, target_language)
  end

  ##
  # Executes a `codify` command and returns the output.
  # Raises an exception if the command fails.
  #
  # @param [String, #to_s] command
  # @param [Array<#to_s>] args
  # @return [String]
  # @raise [Error::NotAvailable]
  # @raise [Error::UnexpectedExit]
  def self.execute(command, *args)
    begin
      out, err, status = Open3.capture3(PROGRAM, command.to_s, *args.map(&:to_s))
    rescue Errno::ENOENT
      raise Error::NotAvailable.new
    end
    case status.exitstatus
      when 0 then out.rstrip
      else raise Error::UnexpectedExit.new(status.exitstatus, err.rstrip)
    end
  end

  ##
  # A collection of errors raised by the `Codify` module.
  class Error < StandardError
    ##
    # The `codify` program is not available.
    class NotAvailable < Error
      def initialize
        super "The `#{PROGRAM}` program is not available."
      end
    end # NotAvailable

    ##
    # The `codify` program exited unexpectedly.
    class UnexpectedExit < Error
      ##
      # @param [Integer, #to_i] code
      # @param [String, #to_s] message
      def initialize(code, message)
        super "The `#{PROGRAM}` program exited with code #{code.to_i}: #{message}"
      end
    end # UnexpectedExit
  end # Error
end # Codify
