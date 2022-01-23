using ExecutableVanillaApplication;

Console.WriteLine("Hello, World!");

var evaEndopint = new EvaEndpoint();

void DoDebug(string taskId, string entity)
{
	evaEndopint.UpdateTaskStatus(taskId);

	// Do work

	evaEndopint.UpdateTaskStatus(taskId);
}

while (true)
{
	switch (evaEndopint.Read())
	{
		case RunTaskRequest { TaskName: "Open window" }:
			break;
		case RunTaskRequest { TaskName: "Debug", TaskId: var taskId,  Parameters: var parameters }:
			DoDebug(taskId, parameters["entity"]);
			break;
		case TaskParameterSuggestionRequest:
			break;
	}

	evaEndopint.UpdateTaskStatus(null);

	Thread.Sleep(1000);
}