using System.Collections.Generic;

namespace ExecutableVanillaApplication;

public abstract record class ManagerMessage;

public sealed record class Parameter(string Name, string Value);

public sealed record class TaskParameterSuggestionRequest : ManagerMessage;
public sealed record class RunTaskRequest(string TaskName, string TaskId, Dictionary<string, string> Parameters) : ManagerMessage;
