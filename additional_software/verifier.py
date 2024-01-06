# Used for running the provided verifier on the solutions created
import subprocess
from os import listdir

print(" Starting verification ")

base_path = "../additional_problems"

for folder in listdir(base_path):

  folder_path = base_path + "/" + folder

  # Find domain file path
  domain_file_name = listdir(folder_path + "/domains")[0]
  domain_file_path = folder_path + "/domains/" + domain_file_name

  # For each solution file, find corresponding problem file
  for solution in listdir(folder_path + "/solutions"):

    solution_path = folder_path + "/solutions/" + solution
    solution_name = solution.split(".solution")[0]

    for problem in listdir(folder_path + "/problems"):
      if problem.__contains__(solution_name):
        problem_path = folder_path + "/problems/" + problem 

        # print(domain_file_path, "\n", solution_path, "\n", problem_path, "\n")

        # Run the verifier with the found arguments
        output = subprocess.run(["pandaPIparser/./pandaPIparser", "-v", domain_file_path, problem_path, solution_path], capture_output=True, text=True)

        #print(output)

        if output.stdout.__contains__("Plan verification result: \x1b[1;32mtrue"):
          print("Verified plan ", problem_path)